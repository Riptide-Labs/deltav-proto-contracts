// Copyright (C) 2026 The Delta-V Authors
// SPDX-License-Identifier: Apache-2.0
//
// Guards the two wire-level invariants that flow_envelope.proto documents in
// prose. Both are load-bearing and both fail silently if they regress: a
// tenancy override that stops overriding is a tenancy bypass, and a presence
// distinction that collapses turns "not reported" into a real zero.

use deltav_proto_contracts::flows::v1::{FlowEnvelope, Tenancy};
use prost::Message;

/// The gateway stamps tenancy by APPENDING its own encoding to the bytes it
/// received, without decoding them. Parsing a concatenation of two encoded
/// messages is equivalent to parsing them separately and merging, so the
/// gateway's values win.
///
/// The subtle part is the empty field. Singular submessages merge recursively
/// rather than being replaced, so an omitted gateway field would leave the
/// producer's claim standing. `Tenancy`'s fields are `optional` precisely so
/// the gateway can write a deliberate empty value and still override.
#[test]
fn gateway_tenancy_append_overrides_producer_claim() {
    // A hostile producer asserts tenancy it is not entitled to.
    let claimed = FlowEnvelope {
        payload: b"datagram".to_vec(),
        tenancy: Some(Tenancy {
            tenant: Some("victim-tenant".to_string()),
            organisation: Some("victim-org".to_string()),
            zone: Some("victim-zone".to_string()),
            system: Some("victim-system".to_string()),
        }),
        ..Default::default()
    };

    // The gateway stamps the authenticated tenancy. `organisation` is
    // deliberately empty: the authenticated identity has no organisation, and
    // that absence must overwrite the claim rather than let it through.
    let stamp = FlowEnvelope {
        tenancy: Some(Tenancy {
            tenant: Some("real-tenant".to_string()),
            organisation: Some(String::new()),
            zone: Some("real-zone".to_string()),
            system: Some("real-system".to_string()),
        }),
        gateway_received_at_unix_nanos: 1_700_000_000_000_000_000,
        ..Default::default()
    };

    // Concatenation, not re-encoding. This is what the gateway does.
    let mut wire = claimed.encode_to_vec();
    wire.extend_from_slice(&stamp.encode_to_vec());

    let merged = FlowEnvelope::decode(&wire[..]).expect("concatenation must parse");
    let tenancy = merged.tenancy.expect("tenancy must survive the merge");

    assert_eq!(tenancy.tenant.as_deref(), Some("real-tenant"));
    assert_eq!(tenancy.zone.as_deref(), Some("real-zone"));
    assert_eq!(tenancy.system.as_deref(), Some("real-system"));
    assert_eq!(
        tenancy.organisation.as_deref(),
        Some(""),
        "an explicitly empty gateway field must overwrite the producer's claim, \
         not be skipped as a default"
    );

    // The payload is untouched by the append, and the gateway timestamp lands.
    assert_eq!(merged.payload, b"datagram");
    assert_eq!(
        merged.gateway_received_at_unix_nanos,
        1_700_000_000_000_000_000
    );
}

/// A datagram carrying no tenancy at all must not gain one from the merge, and
/// the gateway's stamp must still apply cleanly.
#[test]
fn gateway_tenancy_append_works_without_a_prior_claim() {
    let honest = FlowEnvelope {
        payload: b"datagram".to_vec(),
        ..Default::default()
    };
    assert!(honest.tenancy.is_none());

    let stamp = FlowEnvelope {
        tenancy: Some(Tenancy {
            tenant: Some("real-tenant".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut wire = honest.encode_to_vec();
    wire.extend_from_slice(&stamp.encode_to_vec());

    let merged = FlowEnvelope::decode(&wire[..]).expect("concatenation must parse");
    assert_eq!(
        merged.tenancy.and_then(|t| t.tenant).as_deref(),
        Some("real-tenant")
    );
}

/// `Tenancy` fields carry explicit presence, so "not reported" and "reported
/// as empty" stay distinguishable across a round trip. Without `optional` both
/// would encode as nothing and decode as the default.
#[test]
fn tenancy_distinguishes_absent_from_empty() {
    let absent = Tenancy::default();
    let empty = Tenancy {
        tenant: Some(String::new()),
        ..Default::default()
    };

    assert_ne!(
        absent.encode_to_vec(),
        empty.encode_to_vec(),
        "absent and explicitly-empty must differ on the wire"
    );

    let absent = Tenancy::decode(&absent.encode_to_vec()[..]).unwrap();
    let empty = Tenancy::decode(&empty.encode_to_vec()[..]).unwrap();

    assert_eq!(absent.tenant, None);
    assert_eq!(empty.tenant, Some(String::new()));
}

/// Field numbers 1-10 are frozen against udpramp's envelope.proto. Protobuf
/// matches by tag, not by type name, so these numbers are the entire
/// compatibility contract between the two schemas. Encoding a known value and
/// asserting the tag byte catches a renumbering that a name-based review would
/// not.
#[test]
fn frozen_field_numbers_match_the_udpramp_envelope() {
    // Tag 6 (payload), wire type 2 (length-delimited) => (6 << 3) | 2 = 0x32.
    let envelope = FlowEnvelope {
        payload: b"x".to_vec(),
        ..Default::default()
    };
    assert_eq!(envelope.encode_to_vec(), vec![0x32, 0x01, b'x']);

    // Tag 8 (ramp_sequence), wire type 0 (varint) => (8 << 3) | 0 = 0x40.
    let envelope = FlowEnvelope {
        ramp_sequence: 1,
        ..Default::default()
    };
    assert_eq!(envelope.encode_to_vec(), vec![0x40, 0x01]);

    // Tag 11 (ramp_exporter_sequence), the first post-freeze addition.
    let envelope = FlowEnvelope {
        ramp_exporter_sequence: 1,
        ..Default::default()
    };
    assert_eq!(envelope.encode_to_vec(), vec![0x58, 0x01]);
}
