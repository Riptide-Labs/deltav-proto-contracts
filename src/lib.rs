pub mod telemetry {
    pub mod v1 {
        tonic::include_proto!("deltav.telemetry.v1");
    }
}

pub mod nodes {
    pub mod v1 {
        tonic::include_proto!("deltav.nodes.v1");
    }
}

pub mod alarms {
    pub mod v1 {
        tonic::include_proto!("deltav.alarms.v1");
    }
}
