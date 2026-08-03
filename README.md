# Delta-V Protobuf Contracts (`deltav-proto-contracts`)

[![CI](https://github.com/Riptide-Labs/deltav-proto-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Riptide-Labs/deltav-proto-contracts/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Riptide-Labs/deltav-proto-contracts?sort=semver)](https://github.com/Riptide-Labs/deltav-proto-contracts/releases/latest)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

The canonical **Apache-2.0 licensed Protocol Buffer schemas and gRPC contracts** for OpenNMS Delta-V multi-protocol telemetry, streaming metrics, node contexts, and alarm events.

Delta-V services do not define their own wire formats. They consume the artifacts published here, so every service speaks the same schema and a contract change is reviewed in one place.
This repository replaces the legacy `deltav-timeseries.proto` with the Prometheus-aligned `telemetry_metric.proto` schema and the gRPC `TelemetryIngestionService`.

## Contracts

- **Telemetry metrics** (`deltav.telemetry.v1.TelemetryMetricBatch`) — Prometheus-compatible dimensional vector model: `name`, `double value`, `map<string, string> labels`, `MetricProvenance`.
- **Protocol label** — the `protocol` label preserves the source: `gnmi-dial-in`, `gnmi-dial-out`, `snmp`, `prom_node-exporter`, `gnmi-cisco-mdt`, `udp-notif-yang-push`, `bmp`.
- **Ingestion** (`deltav.telemetry.v1.TelemetryIngestionService`) — HTTP/2 client-streaming gRPC endpoint for edge collectors.
- **Node context** (`deltav.nodes.v1.NodeContext`) — metadata schema for Kafka GlobalKTable topology joins.
- **Alarm lifecycle** (`deltav.alarms.v1.AlarmEvent`) — fault alarm state management.

## Quick start

### Java (Maven)

```xml
<dependency>
    <groupId>org.deltav.contracts</groupId>
    <artifactId>deltav-proto-contracts</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Rust (Cargo)

```toml
[dependencies]
deltav-proto-contracts = { git = "https://github.com/Riptide-Labs/deltav-proto-contracts.git", tag = "v1.0.0" }
```

```rust
use deltav_proto_contracts::telemetry::v1::{MetricType, TelemetryMetric, TelemetryMetricBatch};

let batch = TelemetryMetricBatch::default();
let kind = MetricType::Gauge; // prost strips the METRIC_TYPE_ prefix
```

## Building from source

`make` is the entry point for everything. Requires JDK 21, a Rust toolchain, [`protoc`](https://github.com/protocolbuffers/protobuf/releases) and [`buf`](https://buf.build/docs/installation).

```bash
make verify     # lint + build + test, the same gate CI runs
make build      # Java and Rust artifacts
make breaking   # schema backwards-compatibility check
make help       # every target
```

## Documentation

- [CONTRIBUTING.md](CONTRIBUTING.md) — sign-off, AI-assistance disclosure, and what makes a schema change breaking
- [RELEASING.md](RELEASING.md) — versioning, how a release is cut, and how to verify signatures
- [SECURITY.md](SECURITY.md) — reporting a vulnerability

## License

Apache License, Version 2.0. See [LICENSE](LICENSE).
