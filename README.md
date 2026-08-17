# Delta-V Protobuf Contracts

[![CI](https://github.com/Riptide-Labs/deltav-proto-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Riptide-Labs/deltav-proto-contracts/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Riptide-Labs/deltav-proto-contracts?sort=semver)](https://github.com/Riptide-Labs/deltav-proto-contracts/releases/latest)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The canonical **Apache-2.0 licensed Protocol Buffer schemas and gRPC contracts** for Delta-V multi-protocol telemetry, flow records, node contexts, and alarm events.

## Overview

Delta-V services do not define independent wire formats. Instead, all microservices consume the generated language artifacts published from this repository. This guarantees that every service speaks the exact same schema and ensures that any contract modification undergoes centralized compatibility review.

### Key Highlights

- **Single Source of Truth**: All protobuf definitions reside under `proto/deltav/`.
- **Multi-Language Support**: Automated generation of Java (Maven), Rust (`prost`/`tonic`), and Go stubs (`buf`).
- **Compatibility-First**: Enforced backward-compatibility checks via `buf` and `make breaking` in CI.

---

## Schema Contracts

| Package | Proto Path | Key Types & Services | Description |
| :--- | :--- | :--- | :--- |
| `deltav.telemetry.v1` | `proto/deltav/telemetry/v1/` | `TelemetryMetricBatch`, `TelemetryIngestionService` | Prometheus-aligned dimensional metrics vector model (`labels`, `MetricProvenance`) and client-streaming gRPC ingestion endpoint. |
| `deltav.flows.v1` | `proto/deltav/flows/v1/` | `FlowEnvelope`, `NormalisedFlow`, `FlowIngestionService` | Flow envelopes, normalized NetFlow/IPFIX/sFlow flow records, and gRPC ingestion service. |
| `deltav.nodes.v1` | `proto/deltav/nodes/v1/` | `NodeContext`, `CompiledNetworkNodeConfig` | Node metadata for Kafka GlobalKTable topology joins and compiled network node GitOps configurations. |
| `deltav.alarms.v1` | `proto/deltav/alarms/v1/` | `AlarmEvent` | Fault alarm lifecycle state management and event propagation. |

---

## Getting Started

### Java (Maven)

The JAR is not published to a Maven repository yet. Download it from the [latest release](https://github.com/Riptide-Labs/deltav-proto-contracts/releases/latest) and install it into your local repository:

```bash
mvn install:install-file \
    -Dfile=deltav-proto-contracts-0.1.0.jar \
    -DgroupId=org.deltav.contracts \
    -DartifactId=deltav-proto-contracts \
    -Dversion=0.1.0 \
    -Dpackaging=jar
```

Then declare the dependency in your `pom.xml`:

```xml
<dependency>
    <groupId>org.deltav.contracts</groupId>
    <artifactId>deltav-proto-contracts</artifactId>
    <version>0.1.0</version>
</dependency>
```

```java
import org.deltav.telemetry.v1.proto.TelemetryMetricBatch;

TelemetryMetricBatch batch = TelemetryMetricBatch.newBuilder()
    .build();
```

### Rust (Cargo)

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
deltav-proto-contracts = { git = "https://github.com/Riptide-Labs/deltav-proto-contracts.git", tag = "v0.1.0" }
```

Usage in Rust:

```rust
use deltav_proto_contracts::telemetry::v1::{MetricType, TelemetryMetricBatch};

let batch = TelemetryMetricBatch::default();
let kind = MetricType::Gauge; // prost strips the METRIC_TYPE_ prefix
```

### Go (Buf Stubs)

There is no published Go module. Generate the stubs from the schemas instead: `make generate` writes them to `gen/go/`, which is build output and is not committed.

```bash
git clone --branch v0.1.0 https://github.com/Riptide-Labs/deltav-proto-contracts.git
cd deltav-proto-contracts
make generate
```

Copy `gen/go/deltav/...` into your module, or vendor the `proto-<version>.tar.gz` from the release and run `buf generate` against it with your own `buf.gen.yaml`.

---

## Development & Building

> [!IMPORTANT]
> `make` is the sole entry point for building, testing, and linting. CI runs these exact Makefile targets.

### Prerequisites

- **JDK 21**
- **Rust Toolchain** (latest stable)
- **[`protoc`](https://github.com/protocolbuffers/protobuf/releases)**
- **[`buf`](https://buf.build/docs/installation)**

### Common Commands

```bash
make verify     # Run complete verification gate (lint + build + test)
make build      # Compile Java (Maven) and Rust (Cargo) artifacts
make test       # Run Java and Rust test suites
make lint       # Run buf lint, rustfmt, and clippy
make breaking   # Check backwards compatibility against origin/main
make help       # Display detailed target help
```

---

## Repository Structure

```
├── proto/deltav/          # Canonical Protobuf schema definitions
│   ├── telemetry/v1/      # Streaming metrics & ingestion service
│   ├── flows/v1/          # Flow records & flow ingestion service
│   ├── nodes/v1/          # Node context & GitOps node configuration
│   └── alarms/v1/         # Alarm lifecycle events
├── gen/                   # Go & Java stubs generated by buf, not committed
├── src/                   # Rust crate wrapper (include_proto!)
├── pom.xml                # Java Maven build configuration
└── Cargo.toml             # Rust Cargo build configuration
```

---

## Documentation & Governance

- [CONTRIBUTING.md](CONTRIBUTING.md) — DCO sign-off, AI-assistance disclosure, and breaking change rules
- [RELEASING.md](RELEASING.md) — Versioning strategy, release workflow, and signature verification
- [SECURITY.md](SECURITY.md) — Vulnerability reporting policy

---

## License

Distributed under the [Apache License, Version 2.0](LICENSE).

