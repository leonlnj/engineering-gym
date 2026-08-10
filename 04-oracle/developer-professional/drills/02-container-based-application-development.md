# Drill log — 02 Container-Based Application Development

## 2026-08-08 · 10 questions · 8 correct · 0 partial · 2 wrong
Missed:
- Repository naming flat-namespace uniqueness _(recall)_ — chose D (invented a "one repository per top-level path segment" limit); the elimination reasoning given actually supported the correct answer A (tenancy-wide flat-name uniqueness), just attached to the wrong letter. §1.2
- Immutable repositories, no-exception rule _(apply)_ — chose D (invented a semver-pattern scoping to immutability); also misread correct answer A as endorsing a repush, when A actually states the same rejection the user argued for. §3.3

## 2026-08-09 · 10 questions · 10 correct · 0 partial · 0 wrong
Correct after a previous miss: Repository naming flat-namespace uniqueness ✓ (missed 2026-08-08), Immutable repositories no-exception rule ✓ (missed 2026-08-08)

## 2026-08-09 (session 2) · 10 questions · 9 correct · 1 partial · 0 wrong
Missed:
- One digest, many tags _(apply)_ — right letter, but elimination reasoning against the distractor addressed a different mechanic (mutable-tag repointing) than the distractor's actual claim (a fictitious per-digest tag-uniqueness constraint OCIR would enforce). §3.4
Mastered: Immutable repositories no-exception rule (as of 2026-08-09) — confirmed correct twice with no regression (missed 2026-08-08, corrected 2026-08-09 session 1, retested clean on a new sub-angle — pull-side unaffected by immutability — this session).

Author-error note (not a user miss): this session's original Q1 tested whether an IAM policy could scope to a repository-name prefix (e.g. "alpha/*") and was graded assuming no such capability exists. Verified against current OCI docs mid-session: OCIR policies *do* support wildcard prefix matching directly on the flat repository name (`target.repo.name = /prefix-*/`, per Oracle's "Policies to Control Repository Access" docs) — the question's premise was wrong, not the user's answer (B), which was regraded to correct. This is a real, verified gap in the lesson (§1.2 covers flat-naming uniqueness but never mentions policy-side wildcard matching on that same flat name) — candidate for a `> Nuance:` callout if the user wants it added. Does not count toward "Repository naming flat-namespace uniqueness" mastery tracking (different sub-topic: cross-compartment name collision, untouched this session, still needs one clean retest).

Resolved (2026-08-09): user asked to fix the lesson. Added a `> Note:` callout after §1.2's flat-naming Nuance, plus a Limits and Sources row citing the wildcard-match doc.
