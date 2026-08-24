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
(a written audit, answer key visible in the same file). This skill tests the *reader*: it asks one
multiple-choice question, withholds the answer until the user commits to a letter and a reason,
grades strictly against the lesson, reveals the full answer key every time, and tracks misses
across sessions so drilling compounds instead of restarting cold. The exam this track studies for
is multiple-choice, so every question here is too — matching the actual skill being tested:
recognizing the right answer among plausible wrong ones. Recall facts, scenarios, discrimination
pairs, and mechanism questions all still get sourced from the lesson; they're just always
delivered as a 4-option MCQ, never free-recall.

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

While reading the log, identify **mastered topics**: any topic tagged `Mastered:` with no later
`Missed:` entry for it since (a later miss overrides the tag — the topic is no longer read as
mastered), plus any topic that just crossed the bar in this same read — logged as "Correct after a
previous miss" once, and, in a later session, retested (any type) and `correct` again with no
`wrong`/`partial` in between. Feed this list into §A2's mastery rule below.

### A2. Build the question set

Generate fresh from the lesson body — never reuse a prior session's exact phrasing. **Every
question is `eliminate`-type: exactly 4 lettered options, one correct, three real distractors.**
There is no other question type — recall facts, applied scenarios, discrimination pairs, and
mechanism/"why" questions all still get sourced from the lesson (see below), they just always
arrive as a 4-option MCQ rather than free-recall.

**Content sourcing** — draw the underlying fact or scenario for each question from:
- Load-bearing facts: service limits, defaults, required prerequisites, resource-model
  relationships. Mine the lesson's `Limits and Sources` table and its `> ⚠️` callouts first, but
  filter what you pull (see the recall filter below) — not every row is fair game as the *correct
  answer's basis*, though a filtered-out fact can still make a good distractor.
- A short scenario, then "what happens / what's the fix" — the correct option names the fix or
  outcome, the distractors are plausible wrong ones.
- A discrimination pair — which service or setting applies, and what in the requirement forces it.
- The mechanic behind a behavior — the production-reasoning thread.

**The recall filter — apply "shape over number" at selection.** A fact earns a slot **as the
correct answer** only if it changes what you'd *decide or do*. Out of bounds as a correct answer:
- A bare figure for a tooling/environment limit with no decision attached to the specific number
  (e.g. an editor's exact home-directory size or session-timeout minutes) — the underlying concept
  ("treat it as a scratchpad, not durable storage") is still fair game; the exact digits aren't a
  standalone target.
- Deep trivia on a deprecated/retired feature — its exact retirement date, full history. One
  shallow fact ("it's retired, the current answer is X") is enough; anything deeper belongs in a
  distractor instead (below), not the correct answer.

**Distractor sourcing.** Never invent trivia — pull all three wrong options from material the
lesson already provides:
- A wrong mental model the lesson explicitly names and refutes (`> Nuance:` callouts).
- A confusable adjacent term/service the lesson contrasts.
- A deprecated/retired feature as the tempting-but-outdated pick — this is where a retired
  product's identity earns its keep: as a distractor, never as the correct answer (see the recall
  filter above).
- An answer that's correct for a *different* requirement than the one stated — the classic
  exam near-miss.
- The right answer's *name*, paired with the *wrong reason* for it — a distractor that tests
  whether the user understands the actual discriminator, not just pattern-matches the label. E.g.
  for "which deployment strategy fits a real-traffic-subset validation requirement," a distractor
  reading "Blue-green, because it also runs two versions live at once" names a real strategy with
  a real fact attached, but the fact doesn't answer what was actually asked.

**Mastery escalation.** A topic on the mastered list from §A1 still gets a question if it appears
this session, but with distractors sharper and more specific than a first-pass question on that
topic would use — mastery doesn't mean retirement, it means the distractors get harder to rule out.

Rules:
- **Previously-missed items go first.** Anything unresolved in the progress log is re-asked —
  reworded, not copied verbatim, new distractors — before any new material.
- One question asks one thing. Never embed the answer or a hint in the question text or options.
- For each question, silently note its topic and a short model answer (correct option plus why
  each of the other three fails) — do not print this yet; it's your scratch reference for Step A4,
  which reveals it in full after grading.

### A3. Run it — one question at a time

For each question, print exactly:

```
Q<n> of <total> · <topic>
<question text>
A) ...
B) ...
C) ...
D) ...
```

Prompt for both parts of the answer: the letter, and why one specific other option is wrong.

Then **stop and wait for the user's typed answer.** Never print more than one question at once,
and never reveal the answer before grading. If the user types `skip`, mark it a miss with no
partial credit, still print the full reveal (Step A4), and move on. If the user types `stop`, end
the session immediately, skip the reveal for the in-flight question, and proceed to Step A5 with
whatever was answered.

### A4. Grade each answer

Immediately after each answer, print a verdict — `correct` / `partial` / `wrong` — followed by
**two or three lines at most**:

- Name the specific mechanic missed, don't re-lecture the whole topic.
- Cite the lesson section to reread, e.g. `→ §4.2`.
- **Grade on the mechanic, not the phrasing.** `correct` requires the real cause-and-effect —
  named accurately — to be present in the answer, even if it's phrased more briefly, from a
  different angle, or less completely than an ideal model answer would. Reserve `partial` for an
  answer that gestures at the right area without ever naming the actual mechanism, or that names a
  mechanism that's subtly wrong (a different, adjacent concept standing in for the real one) — not
  for an answer that has the right mechanism but doesn't restate every detail of it. A generous
  drill teaches nothing, but a pedantic one just teaches distrust of the grading — five attempts
  at the same item where three already stated the real mechanic is a sign the bar was miscalibrated
  toward the second failure mode, not the first.
- **Grade the shape, not the number.** The consequence — what a limit *forces* — is the
  load-bearing part per `GUIDELINES.md`'s "shape over number" rule, so it's what's graded. Right
  consequence, wrong or missing figure: `correct` under the rule above. Right figure, wrong or
  missing consequence: still `partial`/`wrong` — the number alone was never the point.
- **Grading the two parts.** `correct` requires the right letter *and* an elimination reason that
  correctly identifies why the named distractor fails, even if briefly stated. `partial` is for no
  reasoning at all, or a reason that's actually wrong (misidentifies why the distractor fails, or
  targets the wrong one) — not for a short-but-accurate reason. Mode C's discriminator bar works
  the same way: "Queue, because it needs replay" when replay wasn't the requirement is `wrong`
  because the reasoning is actually inapplicable, not because it's brief.

**Then, always — regardless of verdict, including `correct` — print the full reveal:**

```
Answer: <letter>) <option text>
Why the others are wrong:
B) ...
C) ...
D) ...
```

(List every option other than the correct one, each with one line on the specific flaw — the wrong
mental model, the confused adjacent concept, the near-miss requirement, or the right-name-wrong-
reason trap it represents.) This runs every time, correct or not — the reveal is how the other two
or three distractors the user *didn't* address still teach something, not just the one they wrote
about.

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
- <short item name> — <one line on what was missed>. §<section>
Correct after a previous miss: <item> ✓ (missed <earlier date>)
Mastered: <topic> (as of <date>) — confirmed correct twice with no regression.
```

The `Mastered:` line is appended the session a topic first crosses the bar defined in §A1 — this
is what future sessions scan for, rather than reconstructing the history from misses each time.
Only append it once per topic; a mastered topic that later regresses (missed again) simply starts
accumulating fresh miss entries as normal — it's no longer read as mastered on the next §A1 pass.

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

**Question shape:** same 4-option MCQ format as Mode A (§A2, §A3). State a scenario with the
requirement spelled out but the service or setting withheld — never name the two options being
discriminated between in the question text itself, only in the lettered options. The trade-off
pair supplies two of the four options directly; round out to four with a plausible third and
fourth — a related service from an adjacent module, or the pair's right name paired with the wrong
reason (see §A2's distractor-sourcing list).

**Grading bar:** correct requires naming **the discriminator** — the specific clause in the
requirement that forces the choice — on top of the usual letter-plus-elimination-reason bar from
§A4. Picking the right letter while citing the wrong justification (e.g. "Queue, because it needs
replay" when replay wasn't the requirement) is `wrong`, even though "Queue" was the right answer
for a different reason — same principle as the right-name-wrong-reason distractor type.

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
