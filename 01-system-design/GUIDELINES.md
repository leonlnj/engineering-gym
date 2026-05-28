# 01-system-design Guidelines

## Scope
This folder is for system-design learning exercises and architecture practice.

## Working Style
- Treat each markdown file in this folder as an independent exercise.
- Read the target exercise file first before proposing changes.
- Focus on architecture clarity, assumptions, trade-offs, and operational concerns.

## Preferred Exercise Structure
1. **Title**: `# Exercise <N> - <Topic>`
2. **Objectives**: A numbered list of clear learning goals.
3. **Context**: Conceptual background and the problem statement.
4. **Design**: Architectural details with at least one Mermaid diagram. Prefer both `graph TD` (architecture) and `sequenceDiagram` (flow) when useful.
5. **Setup**: Step-by-step deployment or compilation instructions (Helm, Cargo, Local, etc.).
6. **Test**: Validation steps, demo commands, and expected outputs.
7. **Cleanup**: Strict teardown commands to avoid dangling resources.
8. **References / Appendix**: Links to official documentation or further reading.

## Clarity Checklist
- Explain why a design/tool is chosen, not only how.
- Include trade-offs (e.g., latency vs durability, simplicity vs flexibility).
- Keep commands grouped by phase (Setup/Test/Cleanup).
- For design-only exercises, explicitly state that execution/cleanup is not required.

## Diagrams
- Include at least one focused architecture diagram for non-trivial systems.
- Prefer both architecture (`graph TD`) and flow (`sequenceDiagram`) when useful.
- Keep diagrams simple and directly tied to the explanation.

## Environment Notes (when implementation is included)
- If Kubernetes is involved, verify cluster/app state before and after changes.
- If Rust projects are involved, use the workspace cargo environment and project-specific path.
- If local scripting is involved, keep commands host-runnable and explicit.

## Boundaries
This file applies to content under `01-system-design/`.
For other top-level folders, use that folder’s own local guideline file.

## Clarity Checklist (Apply to Every Exercise)
- Explain **why** a design/tool is chosen, not only **how** to run commands.
- Add trade-offs for major decisions (latency vs durability, simplicity vs flexibility, etc.).
- Keep commands copy-paste ready and grouped by phase (Setup/Test/Cleanup).
- For design-only exercises, keep Setup/Test/Cleanup explicit as `No deployment required`, `No execution required`, etc.
- Prefer concise, purposeful diagrams over decorative diagrams.

## Agent Decision Flow
1. **Identify** the exercise being worked on (via context or user prompt).
2. **Read** the corresponding `exercise/<number>-*.md` file.
3. **Verify State**: E.g., is minikube running? Is the cargo environment sourced? Are previous exercises cleaned up?
4. **Execute**: Follow the setup/test/teardown order specified in the exercise document.
5. **Recover**: If errors occur, check port conflicts, missing environment sourcing (Rust), or pending pod states (`kubectl get pods`).

## Agent Persona & Communication Style
- **Educational & Insightful**: Do not sacrifice meaningful educational details for the sake of extreme conciseness. When introducing a pattern, architecture, or tool, explain *why* it is used and discuss the trade-offs.
- **Proactive Visuals**: Actively suggest and generate new Mermaid diagrams (architecture, sequence, state, etc.) if they can help improve clarity or illustrate complex interactions, even if the user hasn't explicitly asked for them.

