# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

Chainlink is a large Go monorepo for the Chainlink decentralized oracle network. The node software is written in Go, smart contracts in Solidity, and the operator UI in TypeScript/React. It supports multiple chains via a relayer abstraction (EVM, Solana, Cosmos, StarkNet).

## Build & Development Commands

### Go (Node Software)

```bash
# Build the chainlink binary
make build
# or
go build -o chainlink ./core/cmd/chainlink

# Run all Go tests
go test ./...

# Run tests for a specific package
go test ./core/services/job/...

# Run a single test by name
go test -run TestJobORM_CreateJob ./core/services/job/...

# Run tests with the race detector
go test -race ./core/...

# Lint Go code
golangci-lint run
# or
make lint

# Run code generation (mocks, abigen wrappers, sqlc)
go generate ./...
```

Database tests require a running PostgreSQL instance:
```bash
export CL_DATABASE_URL="postgresql://chainlink:chainlink@localhost:5432/chainlink_test?sslmode=disable"
go test ./core/...
```

### Solidity Contracts (`contracts/`)

```bash
cd contracts
pnpm install

# Compile contracts (Hardhat)
pnpm compile

# Run Hardhat tests
pnpm test

# Run Foundry tests
forge test

# Run a specific Foundry test
forge test --match-test testFunctionName

# Generate Go ABI wrappers (run from repo root)
go generate ./core/gethwrappers/...
```

### Operator UI (`operator-ui/`)

```bash
cd operator-ui
pnpm install
pnpm build
pnpm test
```

## Repository Structure

```
core/                        # Main Go node software
  cmd/                       # CLI entry points (chainlink binary)
  services/                  # Core business logic services
    job/                     # Job ORM, spawner, and type registry
    pipeline/                # DAG task pipeline executor
    ocr/                     # OCR1 (Off-Chain Reporting v1)
    ocr2/                    # OCR2 and all OCR2 plugins
    keeper/                  # Automation/Upkeeps (legacy)
    vrf/                     # VRF v1/v2/v2plus
    relay/                   # Multi-chain relayer abstraction
    feeds/                   # Feeds Manager service
    directrequest/           # Direct Request job listener
    fluxmonitor/             # Flux Monitor service
    blockhashstore/          # Block Hash Store feeder
    telemetry/               # Telemetry exporter
  chains/
    evm/                     # EVM chain implementations (clients, txmgr, log poller)
    cosmos/                  # Cosmos chain support
    solana/                  # Solana chain support (via LOOPP plugin)
  web/                       # HTTP server, REST API, GraphQL
    resolver/                # GraphQL resolvers
    presenters/              # REST API presenters
  config/                    # Node configuration (TOML-based v2 config)
  logger/                    # Logging abstractions
  utils/                     # Shared utility packages
  store/migrate/migrations/  # PostgreSQL migration files
  gethwrappers/              # Auto-generated Go ABI contract bindings (DO NOT EDIT)
  internal/testutils/        # Test helpers and utilities

contracts/                   # Solidity smart contracts
  src/                       # Contract source files
  test/                      # Contract tests (Hardhat/Foundry)

deployment/                  # Deployment tooling and CCIP configuration
  ccip/                      # Cross-Chain Interoperability Protocol

integration-tests/           # End-to-end tests (Go + Docker/k8s)
  smoke/                     # Smoke test suites
  soak/                      # Soak/longevity tests
  load/                      # Load tests

plugins/                     # LOOPP external plugins (HashiCorp go-plugin)
operator-ui/                 # React/TypeScript operator node UI
common/                      # Shared Go libraries used across the repo
charts/                      # Helm charts for Kubernetes deployments
docs/                        # Documentation
```

## Key Architectural Concepts

### Jobs and Pipeline

Everything the node executes is a **Job**. Each job has a type (e.g., `directrequest`, `cron`, `webhook`, `keeper`, `ocr`, `ocr2`, `vrfv2`, `offchainreporting2`) and a TOML spec. Most job types execute a **Pipeline** — a DAG of tasks defined in the job spec using TOML. Pipeline task types include `http`, `bridge`, `jsonparse`, `multiply`, `ethabiencode`, `ethdecode`, `median`, etc.

The job lifecycle: `job.Spawner` reads job specs from the DB and starts/stops the appropriate `job.ServiceCtx` for each job type.

### Relayer Abstraction (LOOPPs)

Multi-chain support uses a **Relayer** interface (`core/services/relay/`). Relayers for EVM are compiled in; Solana, Cosmos, StarkNet, etc. run as external **LOOPP plugins** (HashiCorp go-plugin) in separate processes. The `plugins/` directory contains these. The `RelayerChainInteroperators` manages all active relayers.

### OCR2 / Off-Chain Reporting

OCR2 (`core/services/ocr2/`) is the consensus protocol for oracle data feeds. It has a plugin architecture where different oracle products (price feeds, automation, VRF, CCIP) implement the `ocr2types.ReportingPlugin` interface and are registered in `core/services/ocr2/plugins/`.

### EVM Transaction Manager

`core/chains/evm/txmgr/` handles all EVM transaction submission, gas estimation, nonce management, and rebroadcasting. Never manually send EVM transactions — always go through `TxManager`.

### Configuration

Since v2, configuration is TOML-based. The main config struct is in `core/config/`. Tests use `configtest.NewGeneralConfig`. The node is configured with `config.toml` and `secrets.toml` (sensitive values like DB URL, key passwords).

### Database

PostgreSQL only. Migrations live in `core/store/migrate/migrations/` (numbered `.sql` files). Query code uses a mix of `sqlx` (hand-written queries) and `sqlc`-generated code. Use `pg.Q` / `pg.NewQ` for database queries in services. Never use raw `*sql.DB` directly — use the `sqlutil.DataSource` abstraction.

## Code Generation

Several categories of files are auto-generated — **do not hand-edit**:

- `core/gethwrappers/**/*_wrapper.go` — Go bindings from Solidity ABIs via `abigen`
- Files with `// Code generated ... DO NOT EDIT` headers — generated by `sqlc`, `mockery`, or `protoc`
- Mocks in `*/mocks/*.go` — generated by `mockery`

Re-generate with:
```bash
go generate ./core/gethwrappers/...   # ABI wrappers
go generate ./...                      # All generation targets
```

## Testing Patterns

- Use `testify/require` (stops on failure) vs `testify/assert` (continues) intentionally.
- DB-dependent tests call `pgtest.NewSqlxDB(t)` or `cltest.NewApplicationEVMDisabled(t)` to get a test DB.
- `cltest.NewApplication(t, ...)` spins up a full in-process Chainlink node for integration tests.
- Tests that require a real DB are guarded with `pgtest.SkipIfNoDB(t)` or `require.NotEmpty(t, os.Getenv("CL_DATABASE_URL"))`.
- Integration tests in `integration-tests/` use the `seth` Go client library and spin up real chain infrastructure via Docker.

## Linting & Style

- `golangci-lint` with config in `.golangci.yml` — run before committing.
- Import grouping: stdlib, external, internal (enforced by `goimports`).
- All new public interfaces in `core/services/` should have a corresponding mock in a `mocks/` subdirectory.
- SQL queries should use named parameters (`:name`) rather than positional (`$1`) for readability.
