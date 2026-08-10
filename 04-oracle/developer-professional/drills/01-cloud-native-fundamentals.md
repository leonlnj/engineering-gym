# Drill log — 01 Cloud Native Fundamentals

## 2026-08-03 · 10 questions · 1 correct · 3 partial · 6 wrong
Missed:
- ONS Notifications topic prerequisite _(recall)_ — didn't name the topic requirement at all; answered with generic tenancy/IAM instead. §4.1
- IAM 404-not-403 debugging _(recall)_ — right that it's a permissions issue, missed that it surfaces as a 404 not a 403. §4.1
- build_spec.yaml mechanisms _(recall)_ — exportedVariables direction inverted (said "from the previous stage" instead of forward to later stages); outputArtifacts conflated with the delivery stage's registry mapping. §4.4
- Blue-green prerequisites on OKE _(recall)_ — named instance-group (a separate deployment target) instead of the two pre-created namespaces; didn't name the ingress-annotation traffic switch. §4.6
- vaultVariables timing _(apply)_ — claimed secrets are re-fetched by version mid-run; they're resolved once at run start and never touched again. §4.4
- Delivery-stage tag bug _(apply)_ — answered with generic Kubernetes imagePullPolicy/digest reasoning instead of the artifact-substitution mechanic (fixed tag vs. `${IMAGE_TAG}` placeholder). §4.5
- OKE vs instance-group deployment target _(discriminate)_ — answered with a rollout strategy (canary) instead of a deployment target; also inverted which targets support blue-green/canary. §4.6
- Blue-green vs canary discriminator _(discriminate)_ — landed on the right strategy (canary) but cited the cost accepted ("two live versions") rather than the discriminating requirement (real-traffic evidence on a subset). §4.6
- Commit-hash threading _(why)_ — captured the debugging-in-reverse benefit but missed that it's also what makes factor X (dev/prod parity) hold via promoting the exact same image. §5.2

## 2026-08-03 · 10 questions · 9 correct · 1 partial · 0 wrong
Missed:
- Delivery-stage tag bug _(apply)_ — correctly diagnosed a fixed/reused tag as the cause but didn't name where it lives (the delivery stage / artifact resource mapping to a literal tag instead of `${IMAGE_TAG}`). §4.5
Correct after a previous miss: ONS topic prerequisite ✓ (missed 2026-08-03), IAM 404-not-403 debugging ✓ (missed 2026-08-03), build_spec.yaml mechanisms ✓ (missed 2026-08-03), Blue-green prerequisites on OKE ✓ (missed 2026-08-03), vaultVariables timing ✓ (missed 2026-08-03), OKE vs instance-group target ✓ (missed 2026-08-03), Blue-green vs canary discriminator ✓ (missed 2026-08-03), Commit-hash threading ✓ (missed 2026-08-03)

## 2026-08-04 · 10 questions · 2 correct · 6 partial · 2 wrong
Missed:
- Delivery-stage tag bug _(apply)_ — named "immutable tag" and the right location (artifact resource), but the actual mechanic is a fixed/literal tag not using the `${IMAGE_TAG}` substitution placeholder, not tag immutability. §4.5
- Native repo trigger events _(recall)_ — answered "true"; native repos trigger on push only, PR events require an external connection. §4.7
- Service mesh pillar on OCI _(recall)_ — got Istio-on-OKE right, but didn't name that the obvious first guess was specifically the retired managed OCI Service Mesh product (May 31, 2025), not just "an OCI service" generically. §1.2/§7
- Cloud Shell/Code Editor limits _(recall)_ — didn't name any of the four specific limits (5 GB home, 60-min inactivity timeout, 24-hr max session, ~6-month purge); answered with the conclusion instead of the facts. §7/§6.2
- Twelve-factor backing services _(recall)_ — correctly named factor IV, but gave factor X's (dev/prod parity) example instead of IV's own ("a database or queue is just a URL + credential in config"). §3.1/§3.2
- Reschedule-triggered crash _(apply)_ — named Disposability (IX) but missed the other factor the lesson pairs it with, Processes/statelessness (VI). §3.2
- Canary vs blue-green discriminator _(discriminate)_ — landed on canary correctly but cited the cost accepted (two live versions) rather than the discriminating requirement (real-traffic evidence on a subset) — same confusion as the 2026-08-03 session, which had resolved. §4.6
- Instance-group host constraint _(discriminate)_ — correctly named instance-group as the target but didn't answer the actual question asked: the Compute Instance Run Command plugin + Oracle Linux/CentOS-only host requirement. §7

Nuance-callout candidates (unresolved or regressed across 2+ sessions):
- Delivery-stage tag bug — wrong → partial → partial across all three sessions to date, never once answered fully correctly.
- Canary vs blue-green discriminator — resolved 2026-08-03 (second session), regressed this session with the identical confusion (strategy name vs. its accepted cost).

## 2026-08-05 · 6 of 10 questions (session stopped early) · 2 correct · 1 partial · 3 wrong
Missed:
- Delivery-stage tag bug _(eliminate)_ — right letter (fixed tag, not the `${IMAGE_TAG}` placeholder), but elimination reasoning against the `imagePullPolicy` distractor was too vague ("a symptom") to count — didn't explain why pull policy is irrelevant when the image reference itself never changes. §4.5
- Cloud Shell as a dev environment _(apply)_ — answered with a bare figure ("24 hours") despite the question explicitly asking for reasoning independent of any specific number; missed the actual point (scratchpad by design, not durable, wrong tool regardless of the timer). §6.2
- Twelve-factor backing services _(eliminate)_ — picked option A outright (factor X's dev/prod-parity example), not B (factor IV's own example) — same IV-vs-X conflation as 2026-08-04, now regressed from mis-describing it to picking it as the answer. §3.1/§3.2
- Reschedule-triggered crash _(apply)_ — skipped.
Correct after a previous miss: Native repo trigger events ✓ (missed 2026-08-04), Service mesh pillar on OCI ✓ (missed 2026-08-04)

Nuance-callout candidates (unresolved or regressed across 2+ sessions):
- Delivery-stage tag bug — four sessions now, never once landed fully correct.
- Twelve-factor backing services (IV vs. X) — missed both of its last two appearances (2026-08-04, 2026-08-05), same specific confusion each time, trending worse (description error → wrong pick).

## 2026-08-05 · 1 of 10 questions (session stopped early) · 0 correct · 1 partial · 0 wrong
Missed:
- Delivery-stage tag bug _(why)_ — restated the bug's cause (literal string doesn't pick up the exported tag) rather than the fix's mechanism (substitution happens at deploy time, from a freshly-computed exported variable each run). §4.5

Nuance-callout candidates (unresolved or regressed across 2+ sessions):
- Twelve-factor backing services (IV vs. X) — untested this session; still unresolved from 2026-08-05's earlier regression.

Retired from further drilling (user override, 2026-08-05): Delivery-stage tag bug — 3 of 5 attempts
(08-03 #2, 08-05 eliminate, 08-05 why) already stated the core mechanic (fixed/static tag vs. the
`${IMAGE_TAG}` placeholder resolving dynamically per run); grading was marking down phrasing
completeness rather than absent understanding on those three. Only 08-03 #1 (off-track, generic
imagePullPolicy reasoning) and 08-04 (real "immutable" vs. "fixed" conflation) were genuine gaps.
Considered understood — do not re-ask unless it resurfaces unprompted in a future miss.

## 2026-08-08 · 10 questions · 9 correct · 1 partial · 0 wrong
Missed:
- Twelve-factor backing services (IV vs. X) _(discriminate)_ — right letter, but elimination reasoning against option B just restated the negation ("it's not the same rule") without naming what actually separates the two factors. §3.1/§3.2
Correct after a previous miss: Blue-green vs. canary discriminator ✓ (missed 2026-08-04, correctly rejected the cost-based distractor this time), Cloud Shell/Code Editor as a dev environment ✓ (missed 2026-08-04, 2026-08-05, named the scratchpad mechanic instead of a bare figure), Reschedule-triggered crash ✓ (missed 2026-08-04, skipped 2026-08-05), Instance-group host constraint ✓ (missed 2026-08-04)

Nuance-callout candidates (unresolved or regressed across 2+ sessions):
- Twelve-factor backing services (IV vs. X) — fourth appearance, fourth miss: description error (08-04) → wrong pick (08-04 regression) → wrong pick (08-05) → right pick with hollow reasoning (08-08). Never once fully correct across four sessions; strong callout candidate if it recurs again.

## 2026-08-08 · 10 questions · 9 correct · 1 partial · 0 wrong
Missed:
- Notification topic + subscription requirement _(apply)_ — right letter, but elimination reasoning restated the subscription point (why A is correct) instead of addressing C's actual flaw (wrongly scoping the topic to deployment events only, when it carries both build and deployment events). §4.1
Correct after a previous miss: Twelve-factor backing services (IV vs. X) ✓ (missed 2026-08-04, 2026-08-04 regression, 2026-08-05, 2026-08-08 hollow reasoning — this time named the actual differentiator: config-portability doesn't guarantee the same image is promoted across environments), OKE vs. instance-group target ✓ (retested with sharper distractors, no regression), vaultVariables timing ✓ (retested with sharper distractors, no regression), build_spec.yaml exportedVariables direction ✓ (retested with sharper distractors, no regression)

Nuance-callout candidates (unresolved or regressed across 2+ sessions):
- Twelve-factor backing services (IV vs. X) — resolved this session after 4 prior misses; watch one more clean appearance before retiring as a callout candidate.
