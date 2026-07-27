# engineering-gym

This repo is setup to experiment/practice random exercises that I stumbled upon

## AI-Assisted Development

This repository is set up for AI-assisted work. `CLAUDE.md` holds repo-wide conventions, each track's
`GUIDELINES.md` holds that track's domain parameters, and `.claude/skills/` (`lesson-craft`,
`lesson-eval`) holds the shared authoring and review craft for the `NN-*.md` learning notes. GitHub
Copilot users get the same pointers via `.github/copilot-instructions.md`.

## Setup

- [devcontainer](https://code.visualstudio.com/docs/devcontainers/tutorial)
  - [kubernetes-helm-minikube](https://github.com/devcontainers/templates/tree/main/src/kubernetes-helm-minikube) container.
- Docker desktop
- MacOS

### Rust Setup

Rust official [book](https://doc.rust-lang.org/book/) to learn!

### Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
```

### Install VScode extension

- rust-analyzer
- CodeLLDB
- Mirrod (by MetalBear, to launch debug into cluster without portforward to access db)

### Configure extension

Update project path in `.vscode/settings.json`

```json
{
    "mirrord.promptUsingBinary": false,
    "rust-analyzer.linkedProjects": [
        "./exercise/resources/ex-11/Cargo.toml"
    ]
}
```