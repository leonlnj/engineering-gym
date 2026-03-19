# Engineering Gym AI Agent Instructions

## Project Overview
This is a **hands-on learning repository** for practicing distributed systems and backend engineering concepts. Each exercise (`exercise/*.md`) is a self-contained isolated experiment focusing on specific technologies or patterns.

**Golden Rule**: Always read the specific `exercise/<number>-*.md` file before assisting the user. The exercise markdown is the ultimate source of truth for setup, implementation, and teardown.

## Standard Exercise Format
All exercises MUST strictly follow this consistent Markdown structure to ensure uniformity across the repository:
1. **Title**: `# Exercise <N> - <Topic>`
2. **Objectives**: A numbered list of clear learning goals.
3. **Context**: Conceptual background and the problem statement.
4. **Design**: Architectural details. **MUST** include Mermaid diagrams (`graph TD` for architecture, `sequenceDiagram` for system/data flow).
5. **Setup**: Step-by-step deployment or compilation instructions (Helm, Cargo, Local, etc.).
6. **Test**: Validation steps, demo commands, and expected outputs.
7. **Cleanup**: Strict teardown commands to avoid dangling resources.
8. **References / Appendix**: Links to official documentation or further reading.

## Universal Archetypes & Workflows

### 1. Kubernetes Workflows (Minikube)
- **Environment**: Kubernetes-in-Docker via minikube. Tools available: `kubectl`, `helm`, `docker`.
- **Setup Pattern**: Operator/CRD installation -> Deploy resources -> Port-forward for access.
- **Cleanup Pattern**: **Order matters**. Delete app clusters -> Clean PVCs (`kubectl delete pvc`) -> Uninstall Helm charts -> Remove operators.
- **State**: Ensure a clean cluster slate between exercises. Check for leftover PVCs or Helm releases.

### 2. Rust Workflows
- **Environment**: Independent Rust projects located in `exercise/resources/ex-<N>/`.
- **CRITICAL Setup Pattern**: You MUST source the cargo environment and navigate to the project directory for EVERY command:
  ```bash
  cd /workspaces/engineering-gym/exercise/resources/ex-<N> && \
  source /workspaces/engineering-gym/.cargo/env && \
  cargo <subcommand>
  ```
- **IDE**: Update `.vscode/settings.json` `rust-analyzer.linkedProjects` to point to the current active exercise's `Cargo.toml`.

### 3. Local / Scripting Workflows (Python, etc.)
- Run directly on the host using local virtual environments (like `uv`). No K8s cleanup required.

### 4. System Design Workflows
- Purely architectural exercises. Include Mermaid diagrams for Architecture (`graph TD`) and Data Flow (`sequenceDiagram`). No implementation or teardown required.

## Agent Decision Flow
1. **Identify** the exercise being worked on (via context or user prompt).
2. **Read** the corresponding `exercise/<number>-*.md` file.
3. **Verify State**: E.g., is minikube running? Is the cargo environment sourced? Are previous exercises cleaned up?
4. **Execute**: Follow the setup/test/teardown order specified in the exercise document.
5. **Recover**: If errors occur, check port conflicts, missing environment sourcing (Rust), or pending pod states (`kubectl get pods`).

## Agent Persona & Communication Style
- **Educational & Insightful**: Do not sacrifice meaningful educational details for the sake of extreme conciseness. When introducing a pattern, architecture, or tool, explain *why* it is used and discuss the trade-offs.
- **Proactive Visuals**: Actively suggest and generate new Mermaid diagrams (architecture, sequence, state, etc.) if they can help improve clarity or illustrate complex interactions, even if the user hasn't explicitly asked for them.

