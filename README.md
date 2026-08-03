# Delta-V Protobuf Contracts (`deltav-proto-contracts`)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

This repository contains the canonical **Apache-2.0 licensed Protocol Buffer schemas and gRPC contracts** for OpenNMS Delta-V multi-protocol telemetry, streaming metrics, node contexts, and alarm events.

It replaces legacy `deltav-timeseries.proto` with the cloud-native, Prometheus-aligned **`telemetry_metric.proto`** schema and gRPC **`TelemetryIngestionService`**.

---

## 1. Supported Protocols & Data Models

- **Telemetry Metrics (`deltav.telemetry.v1.TelemetryMetricBatch`)**: Standardized Prometheus-compatible dimensional vector model (`name`, `double value`, `map<string, string> labels`, `MetricProvenance`).
- **Dedicated `protocol` Label**: Preserves protocol source context (`gnmi-dial-in`, `gnmi-dial-out`, `snmp`, `prom_node-exporter`, `gnmi-cisco-mdt`, `udp-notif-yang-push`, `bmp`).
- **Cloud-Native Ingestion (`deltav.telemetry.v1.TelemetryIngestionService`)**: HTTP/2 streaming gRPC endpoint for edge collectors (`tonic` in Rust).
- **Node Context (`deltav.nodes.v1.NodeContext`)**: Metadata schema for Kafka GlobalKTable topology joins.
- **Alarm Lifecycle (`deltav.alarms.v1.AlarmEvent`)**: Protobuf contract for fault alarm state management.

---

## 2. Usage Instructions

### Java (Maven)

Add the dependency to your `pom.xml`:

```xml
<dependency>
    <groupId>org.deltav.contracts</groupId>
    <artifactId>deltav-proto-contracts</artifactId>
    <version>1.0.0-SNAPSHOT</version>
</dependency>
```

Build generated Java classes:
```bash
mvn clean compile
```

### Rust (`Cargo.toml`)

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
deltav-proto-contracts = { git = "https://github.com/pbrane/deltav-proto-contracts.git", tag = "v1.0.0" }
```

In your Rust code:
```rust
use deltav_proto_contracts::telemetry::v1::{TelemetryMetricBatch, TelemetryMetric, MetricType};
```

---

## 3. License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
