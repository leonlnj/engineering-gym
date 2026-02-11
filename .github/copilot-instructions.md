# Engineering Gym AI Agent Instructions

## Project Overview

This is a **hands-on learning repository** for practicing distributed systems and backend engineering concepts. Each exercise (`exercise/*.md`) is a self-contained problem with its own setup, implementation, and teardown instructions.

**Key Principle**: Exercises are isolated experiments, not a cohesive application. Each focuses on a specific technology or pattern (databases, caching, message queues, rate limiting, circuit breakers, etc.).

---

## Environment & Dev Container

- **Runtime**: Kubernetes-in-Docker via minikube inside a Debian 11 devcontainer
- **Auto-start**: minikube starts automatically 20s after container creation (see `.devcontainer/devcontainer.json`)
- **Available tools**: `kubectl`, `helm`, `docker`, `minikube`, standard POSIX utilities
- **Shell access**: Use `run_in_terminal` for kubectl/helm commands; never create nested shells

---

## Rust Projects: Critical Setup Pattern

**Problem**: The workspace contains multiple independent Rust projects under `exercise/resources/ex-{11,12,14}/`, NOT a workspace root `Cargo.toml`.

### Universal Command Template

All Rust commands must follow this pattern:

```bash
cd /workspaces/engineering-gym/exercise/resources/ex-<N> && \
source /workspaces/engineering-gym/.cargo/env && \
cargo <subcommand>
```

### Common Operations

| Task | Command |
|------|---------|
| Build | `cd .../ex-<N> && source .../.cargo/env && cargo build` |
| Run | `cd .../ex-<N> && source .../.cargo/env && cargo run` |
| Run specific binary | `cd .../ex-<N> && source .../.cargo/env && cargo run --bin <name>` |
| Test | `cd .../ex-<N> && source .../.cargo/env && cargo test` |
| Check (fast) | `cd .../ex-<N> && source .../.cargo/env && cargo check` |

**Shortcut**: If already in exercise directory:
```bash
source /workspaces/engineering-gym/.cargo/env && cargo <subcommand>
```

### VSCode `rust-analyzer` Configuration

Update `.vscode/settings.json` when working on a different exercise:

```json
{
  "rust-analyzer.linkedProjects": [
    "./exercise/resources/ex-<N>/Cargo.toml"
  ]
}
```

**Why this matters**: 
- Without sourcing `.cargo/env`, Rust commands fail
- Without navigating to the exercise directory, cargo operates on the wrong project
- rust-analyzer needs explicit project linking for multi-project repos

---

## Kubernetes Workflow Patterns

### Setup Pattern (Most Exercises)

Typical flow:
1. Install Kubernetes operator/controller via `kubectl apply -f <url>` or helm
2. Wait for operator readiness: `kubectl get pod -n <namespace> --watch`
3. Deploy application resources (databases, message queues)
4. Port-forward services for local access: `kubectl port-forward service/<name> <local>:<remote>`

### Database Access Pattern

- Credentials stored in Kubernetes secrets: 
  ```bash
  kubectl get secret <name> -o jsonpath="{.data.password}" | base64 -d
  ```
- PostgreSQL client pod pattern:
  ```bash
  kubectl run postgresql-client --rm --tty -i --restart='Never' \
    --image registry-1.docker.io/bitnami/postgresql:latest \
    --env="PGPASSWORD=$PASSWORD" --command -- psql --host <host> -U <user>
  ```

### Teardown Pattern

**Order matters** to avoid dangling resources:

1. Delete application clusters: `kubectl delete cluster <name>`
2. Clean PVCs: `kubectl get pvc | grep '<pattern>' | awk '{print $1}' | xargs kubectl delete pvc`
3. Uninstall helm charts: `helm uninstall <release>`
4. Remove operators: `kubectl delete -f <operator-url>`

---

## Exercise Documentation Structure

All exercises follow a **standardized documentation pattern** for consistency and ease of navigation.

### Standard Exercise Format

Every `exercise/<number>-*.md` file follows this structure:

```markdown
# Exercise N - [Technology/Pattern Name]

[Brief overview paragraph]

**Objectives**:
1. [Learning goal 1]
2. [Learning goal 2]
...

## Context (optional)
[Conceptual background, comparisons, architectural considerations]

## Setup
[Installation/deployment steps, prerequisites]

## Test
[Validation steps, demo commands, verification procedures]

## Cleanup
[Teardown commands, resource removal steps]

## Appendix (optional)
[Supplementary information, alternatives, troubleshooting]
```

### Exercise Categories

**Kubernetes-based exercises** (1-6, 8, 10-12):
- Use helm/kubectl for deployment
- Require minikube cluster
- Need port-forwarding for local access
- Must clean up PVCs and helm releases

**Local Python development exercises** (7, 9):
- Run on host without Kubernetes
- Use local Python/virtual environments
- No cluster resources to clean up

**Rust exercises** (11, 12, 14):
- Located in `exercise/resources/ex-<N>/`
- Run locally without Kubernetes (no namespace / no K8s deployment)
- Require sourcing `.cargo/env` before all commands
- Need rust-analyzer configuration updates
- May use dual binary pattern (server + client)

**Design-only exercises** (13):
- RFC/architecture documentation
- No implementation or deployment
- No setup/teardown required

---

## Exercise-Specific Quick Reference

| Ex | Focus | Type | Key Tech | Namespace | Special Notes |
|----|-------|------|----------|-----------|---------------|
| 1  | Relational DB | K8s | PostgreSQL | default | Transactions, isolation levels |
| 2  | Caching | K8s | Redis | default | Inline Python demo |
| 3  | Message Queue | K8s | RabbitMQ | default | Destructive reads |
| 4  | Message Stream | K8s | Kafka/Strimzi | kafka | Immutable log |
| 5  | Pub/Sub vs Streams | K8s | Redis | default | Compare patterns |
| 6  | Circuit Breaker | K8s | Python/pybreaker | default | Ephemeral pod demo |
| 7  | WebSocket | Local | Socket.IO | - | Python server/client |
| 8  | Bloom Filters | K8s | Redis Stack | default | Probabilistic DS |
| 9  | Consistent Hash | Local | Python | - | Pure algorithm |
| 10 | Real-time ETL | K8s | Kafka/Flink | kafka | Custom Docker image |
| 11 | E-commerce API | K8s+Rust | CNPG/Axum | default | Read/write splitting |
| 12 | Rate Limiting | K8s+Rust | Redis/Axum | default | Multiple strategies |
| 13 | Notification System | Design | - | - | RFC only |
| 14 | Content Masking | Rust | WebSocket/Trie | - | Dual binary pattern |

### Common Exercise Patterns

**Pattern: Inline Kubernetes Demo** (Ex 2, 6)
```bash
kubectl run <name> --rm -i --restart=Never --image=<image> --command -- sh -c '...'
```
- No persistent files
- Auto-cleanup with `--rm` flag
- Good for quick prototyping

**Pattern: Helm + Port-forward** (Ex 1-5, 8, 12)
```bash
helm install ex-<N> <chart> [--set options]
kubectl port-forward service/<name> <port>:<port>
# ... test ...
helm uninstall ex-<N>
kubectl get pvc | grep '<pattern>' | xargs kubectl delete pvc
```

**Pattern: Operator + CRD** (Ex 4, 10, 11)
```bash
kubectl apply -f <operator-url>
kubectl get pod -n <namespace> --watch
kubectl apply -f <resource.yaml>
# ... test ...
kubectl delete -f <resource.yaml>
kubectl delete -f <operator-url>
```

**Pattern: Rust Application** (Ex 11, 12, 14)
```bash
cd /workspaces/engineering-gym/exercise/resources/ex-<N>
source /workspaces/engineering-gym/.cargo/env
cargo run [--bin <name>] [-- <args>]
# ... test ...
# Ctrl+C to stop
```

---

## Exercise-Specific Implementation Notes

**Before helping with an exercise**:
1. Read `exercise/<number>-*.md` for complete setup/teardown procedure
2. Check exercise category (K8s/Local/Rust/Design) from table above
3. Verify prerequisites (minikube status for K8s, cargo env for Rust)
4. For Rust: Confirm `Cargo.toml` location in `exercise/resources/ex-<number>/`

### Detailed Exercise Notes


**Only reference this section for exercise-specific details not covered by the standard patterns above.**

#### Ex 1: Relational Database (PostgreSQL)
- Secret extraction: `kubectl get secret ex-1-postgresql -o jsonpath="{.data.postgres-password}" | base64 -d`
- Client pod: `kubectl run ex-1-postgresql-client --rm -ti --restart='Never' --image bitnami/postgresql --env="PGPASSWORD=$PASSWORD" --command -- psql --host ex-1-postgresql -U postgres`
- Tests: Triggers, constraints, cascading deletes, isolation levels

#### Ex 3: Message Queue (RabbitMQ)
- Uses legacy image: `--set image.repository=bitnamilegacy/rabbitmq`
- Requires `rabbitmqadmin` CLI tool downloaded locally
- Management UI: Port-forward 15672
- Exchange types: direct, fanout, topic, headers

#### Ex 4: Message Stream (Kafka)
- **Namespace**: Always use `kafka` namespace
- Two-terminal pattern: Producer in one, consumer in another
- Uses Strimzi operator with single-node cluster CRD

#### Ex 7: WebSocket with Socket.IO
- Requires `uv` Python environment: `uv venv websocket -p 3.14`
- Install: `uv pip install python-socketio uvicorn websocket-client`
- Files: `server.py` (Uvicorn/AsyncIO), `client.py` (event-driven)

#### Ex 8: Bloom Filters
- **Important**: Use `redis-stack` helm repo, NOT bitnami (RedisBloom module required)
- False positives possible, false negatives impossible
- Commands: `BF.RESERVE`, `BF.ADD`, `BF.EXISTS`

#### Ex 9: Consistent Hashing
- Pure Python, no external dependencies
- Two-array implementation: `ring_pos` (sorted positions), `nodes` (server names)
- Uses MD5 for stable hashing across sessions
- Virtual nodes via `replicas` parameter

#### Ex 10: Big Data ETL (Kafka + Flink)
- **Namespace**: Uses `kafka` namespace
- Custom Docker image: Build with `dockerfile`, load into minikube
- PyFlink job in `job.py`, deployment in `flink.yaml`
- Application Mode: 1 cluster = 1 job

#### Ex 11: E-commerce API (CNPG + Rust)
- **Read/write splitting**: `rw_pool` (primary), `ro_pool` (replica) - see `main.rs:45-49`
- Hardcoded credentials in `main.rs:37-38` (learning environment only)
- Transaction example: `rw_pool.begin().await` in `add_product` function
- Port-forward both: Primary 5432, replica different port

#### Ex 12: Rate Limiting (Redis + Rust)
- **CLI args**: `cargo run -- <strategy> <limit> <window_secs>`
- Example: `cargo run -- fixed 3 10` (3 requests per 10 seconds)
- Strategies: fixed window, sliding window
- Redis must be at `localhost:6379`

#### Ex 14: Content Masking (WebSocket + Rust)
- **Dual binary**: `cargo run --bin server` and `cargo run --bin client` (separate terminals)
- Trie loaded from `abuse.txt` at startup
- Performance: `unsafe` `String::from_utf8_unchecked` for ASCII masking (see `server.rs:46`)
- Concurrency: One tokio task per WebSocket connection with shared `Arc<Trie>`
- Server port: 8080 (verify in source if connection fails)

*When adding new exercises: Add only critical details here that differ from standard patterns.*

---

## Verification Patterns

### After Operator Installation

```bash
kubectl wait --for=condition=available --timeout=300s deployment/<name> -n <namespace>
```

### After Database/Service Deployment

```bash
kubectl get <resource-type> <name> -o jsonpath='{.status.phase}'  # Should show "Running" or "Ready"
```

### After Port-Forward (Non-blocking)

```bash
kubectl port-forward service/<name> <port>:<port> > /dev/null 2>&1 &
sleep 2 && nc -zv localhost <port>  # Verify port is open
```

### Before Rust Build

```bash
rustc --version && cargo --version  # Confirm toolchain availability
```

### After Teardown

```bash
# Verify cleanup
kubectl get pvc | grep 'ex-<N>'  # Should return nothing
helm list -A | grep 'ex-<N>'     # Should return nothing
kubectl get pods -A | grep 'ex-<N>'  # Should return nothing
```

---

## Cluster State Management

**Golden rule**: Each exercise should start with a clean slate.

### Before Starting a New Exercise

```bash
# Check for leftover resources
kubectl get all --all-namespaces | grep -E 'ex-[0-9]+'
helm list --all-namespaces

# If found, run teardown for previous exercise (see exercise/<number>-*.md)
```

### Persistent State Across Exercises

- **minikube cluster**: Shared across exercises (DO NOT delete unless corrupted)
- **PVCs**: Exercise-specific; must be cleaned during teardown
- **Helm releases**: Must be uninstalled per exercise
- **CRDs**: Some operators leave CRDs behind; may need `kubectl delete crd <name>`

### If Minikube Needs Reset

```bash
minikube stop
minikube delete
minikube start  # Will take 2-3 minutes to initialize
```

---

## Common Error Patterns & Recovery

### "cargo: command not found"

- **Cause**: `.cargo/env` not sourced
- **Fix**: Prepend `source /workspaces/engineering-gym/.cargo/env &&` to command

### "error: could not find `Cargo.toml`"

- **Cause**: Wrong directory
- **Fix**: Navigate to `exercise/resources/ex-<number>/` first

### "connection refused" to Kubernetes Service

- **Cause**: Port-forward not active or minikube not ready
- **Fix**: 
  1. Check minikube: `minikube status`
  2. Verify pod: `kubectl get pod -n <namespace>`
  3. Check pod logs: `kubectl logs <pod-name> -n <namespace>`
  4. Restart port-forward: `kubectl port-forward service/<name> <port>:<port> &`

### "ImagePullBackOff" on Operator Installation

- **Cause**: Slow registry or network issue inside devcontainer
- **Fix**: Wait 2-3 minutes, then check: `kubectl describe pod <name> -n <namespace>`
- **If persistent**: Check minikube logs and network connectivity

### rust-analyzer Errors in VSCode but Cargo Builds Fine

- **Cause**: `.vscode/settings.json` pointing to wrong exercise
- **Fix**: Update `rust-analyzer.linkedProjects` to current exercise's `Cargo.toml`

### Port Already in Use

- **Cause**: Previous port-forward still running
- **Fix**: 
  ```bash
  lsof -ti:<port> | xargs kill -9  # Kill process using port
  # Or restart port-forward with different local port
  kubectl port-forward service/<name> <new-port>:<remote-port> &
  ```

### "context deadline exceeded" During kubectl Operations

- **Cause**: Minikube not fully initialized or resource constraints
- **Fix**: 
  1. Wait 30-60 seconds and retry
  2. Check minikube status: `minikube status`
  3. Check container resources (CPU/memory)

---

## Agent Decision Flow

### When User Asks for Help with an Exercise

1. **Identify exercise number** from user message or file paths
2. **Read exercise guide**: `exercise/<number>-*.md` (if exists)
3. **Check environment readiness**:
   - Minikube running? (`minikube status`)
   - Previous exercise cleaned up? (check PVCs, helm releases)
4. **For Rust exercises**: Verify Cargo environment first
5. **Follow setup order**: Operators → App resources → Port-forwards → Implementation
6. **After success**: Suggest teardown steps and cleanup verification

### When Debugging

1. **Check common error patterns** section first
2. **Apply error recovery patterns** before suggesting workarounds
3. **Verify state** after each fix attempt using verification patterns
4. **Reference exercise markdown** for exercise-specific details
5. **Check recent container/minikube logs** if infrastructure issues suspected

### When User Mentions...

- **"build failed" or "cargo error"** → Check Cargo environment sourcing first (see Rust Setup Pattern)
- **"connection refused" or "cannot connect"** → Verify port-forward and minikube status
- **"exercise X" or "ex-X"** → Read `exercise/X-*.md` for specific setup steps before suggesting commands
- **"clean up" or "teardown"** → Follow teardown order (app → PVCs → helm → operators)
- **"rust-analyzer not working"** → Check `.vscode/settings.json` linked project
- **"stuck" or "waiting"** → Check for operator readiness (`kubectl get pod -n <namespace> --watch`)
- **Multiple exercises in one session** → Remind about cluster state cleanup between exercises
- **"starting over" or "reset"** → Verify teardown completed before new exercise setup

---

## File Locations (Quick Reference)

- **Exercise guides**: `exercise/<number>-*.md`
- **Rust projects**: `exercise/resources/ex-<number>/Cargo.toml`
- **VSCode settings**: `.vscode/settings.json`
- **Devcontainer config**: `.devcontainer/devcontainer.json`
- **Minikube logs**: Check container logs if kubectl fails
- **This instruction file**: `.github/copilot-instructions.md`

---

## Tool Versions (Reference)

- **Rust edition**: 2024 (requires recent stable toolchain)
- **Kubernetes**: minikube default version
- **Container base**: Debian 11
- **kubectl/helm**: Versions bundled with devcontainer

*Update this section if exercises require specific tool versions.*

---

## Extension Configuration

- **Mirrord**: Configured for in-cluster debugging without local port-forwarding (see `.vscode/settings.json`)
- **rust-analyzer**: Manually link projects per exercise (not auto-detected)
- **CodeLLDB**: Available for Rust debugging

---

## Common Gotchas

1. **Minikube readiness**: If kubectl commands fail after container start, wait 30-60s for minikube (check `minikube status`)
2. **Port conflicts**: Ensure port-forwards don't collide (Redis 6379, PostgreSQL 5432, WebSocket 8080)
3. **Hardcoded credentials**: Exercises use placeholder passwords for learning; replace before any production-like use
4. **Rust edition**: Projects use `edition = "2024"` (forward-looking; may require recent toolchain)
5. **PVC persistence**: PVCs survive pod deletion; must be manually cleaned during teardown
6. **CRD leftovers**: Some operators leave CRDs after uninstall; check with `kubectl get crd`
7. **Resource limits**: Minikube in container has limited resources; large deployments may be slow

---

## Best Practices for Exercises

### Setup Phase
- Always read the exercise markdown guide first
- Verify minikube status before starting
- Check for leftover resources from previous exercises
- Follow setup steps in exact order

### Implementation Phase
- For Rust: Always source Cargo environment and navigate to exercise directory
- Test incrementally (build → check → run)
- Use verification commands after each major step
- Keep port-forwards in background and verify connectivity

### Teardown Phase
- Follow teardown order strictly
- Verify cleanup with verification commands
- Check for orphaned PVCs and helm releases
- Document any issues encountered for future reference

---

## When Helping Users

- **Always check exercise number** in file paths or user context to apply exercise-specific patterns
- **Verify minikube state** before suggesting kubectl commands
- **Remind about Cargo environment** when debugging Rust build failures
- **Reference exercise markdown** (e.g., `11-design-ecommerce-product-listing-cnpg-rust.md`) for complete setup steps
- **Suggest teardown commands** after successful exercise completion to clean cluster state
- **Provide verification commands** after suggesting fixes to confirm success
- **Check for common errors** before suggesting complex debugging
- **Encourage incremental testing** rather than running entire setup at once

---

## Notes for Future Exercises

When adding new exercises:

1. **Follow standard documentation structure**:
   - Title: `# Exercise N - [Technology/Pattern]`
   - Add brief overview + **Objectives** list
   - Use sections: Context → Setup → Test → Cleanup (+ optional Appendix)

2. **Update quick reference table** in this file with:
   - Exercise number and focus area
   - Type (K8s/Local/Rust/Design)
   - Key technologies
   - Namespace (if K8s-based)
   - Special notes (1-2 words max)

3. **Add detailed notes only if**:
   - Exercise deviates significantly from standard patterns
   - Special prerequisites or configurations required
   - Non-obvious gotchas that will block users

4. **File organization**:
   - Markdown guide: `exercise/<number>-<name>.md`
   - Resources (if needed): `exercise/resources/ex-<number>/`
   - Rust projects: Must have `Cargo.toml` in resources directory

5. **Test teardown thoroughly**:
   - Verify all PVCs are removed
   - Check for orphaned helm releases
   - Ensure namespaces clean up correctly
   - Document any manual cleanup steps required

6. **Categorize correctly**:
   - K8s-based: Uses helm/kubectl, requires minikube
   - Local: Runs on host without cluster resources
   - Rust: Requires cargo environment sourcing
   - Design: Documentation only, no implementation

This structure scales to 50+ exercises without requiring major reorganization.