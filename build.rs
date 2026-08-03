// Copyright (C) 2026 The Delta-V Authors
// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/deltav/telemetry/v1/telemetry_metric.proto",
                "proto/deltav/telemetry/v1/telemetry_ingestion_service.proto",
                "proto/deltav/nodes/v1/node_context.proto",
                "proto/deltav/alarms/v1/alarm_event.proto",
                "proto/deltav/flows/v1/flow_envelope.proto",
                "proto/deltav/flows/v1/flow_ingestion_service.proto",
                "proto/deltav/flows/v1/normalised_flow.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
