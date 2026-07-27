# 04-oracle — How to Study This Track

Two Oracle Cloud Infrastructure (OCI) certifications, each as its own sub-track with its own study plan:

- `developer-professional/` — OCI Developer Professional
- `observability-professional/` — OCI Observability Professional

Both share one `GUIDELINES.md` (audience, snippet languages, trade-off pairs) at this level. This README is about *method*: how to use the repo's tooling so the studying compounds instead of evaporating.

## The loop this repo is built for

The repo's two skills form a write/grade pair, and the order below is what makes the pair honest:

1. **Fill in `GUIDELINES.md` first.** Both skills read it before doing anything; the TODOs mark the parameters only you can supply.
2. **Write the lesson's `STUDY-PLAN.md` paragraph before writing the lesson** — taken from Oracle's official exam blueprint, not from what you already know. That paragraph is the spec `lesson-eval` grades against.
3. **Draft the lesson with `/lesson-craft`.** It enforces the depth bar (walkthroughs, snippets, diagrams, the why behind each mechanism) so notes come out as reference material, not paraphrased slides.
4. **Run `/lesson-eval` (quiz mode) when the lesson feels done.** It generates a quiz *blind* — from the study-plan spec, before reading your lesson — then grades the lesson against it. Gaps it finds are things the exam could ask that your notes can't answer.
5. **Close the gaps, then run review mode** to audit the prose itself.

## Why blueprint-first matters

If you write the lesson first and the spec second, the spec describes what you wrote — and every quiz confirms it. Anchoring each study-plan paragraph to the published exam topics keeps the bar *outside* your own notes: coverage quizzes then measure exam-readiness, not self-agreement. Copy the blueprint's topic wording into the spec paragraphs; rephrase later if it reads awkwardly.

## Use the quiz bank for spaced retrieval

Every coverage run adds a **new, deliberately different** quiz to `<sub-track>/assessments/<lesson-slug>/` — the bank is designed to probe the topic from rotating angles. That makes it a ready-made spaced-repetition deck:

- A few days after finishing a lesson, answer its oldest quiz **cold** — write answers before re-opening the lesson, then check against the answer key.
- A week or two out, run `/lesson-eval` again for a fresh quiz. Repeated retrieval with feedback is what moves material into long-term memory; rereading alone does not.
- Anything you miss twice is a candidate for its own `> Nuance:` callout back in the lesson.

## Anchor every lesson in a real tenancy

Cert questions reward people who have *operated* the service, not just read about it. Pair each lesson with OCI Free Tier practice: reproduce the lesson's worked walkthrough against a real tenancy (deploy the function, fire the alarm, trace the request) before calling the lesson done. The friction you hit doing it is usually exactly what the exam's scenario questions probe.

## Suggested order

Do `developer-professional/` first: the observability exam assumes you can already build and deploy the workloads you're being asked to monitor, so its vocabulary lands better second. If an exam booking dictates otherwise, swap — the sub-tracks are self-contained.
