pub mod google {
    pub mod protobuf {
        pub use prost_types::*;
    }
}

pub mod protos {
    include!("protos/protos.rs");
    include!("protos/protos.tonic.rs");
}

pub mod logserver {
    include!("logserver/logserver.rs");
    include!("logserver/logserver.tonic.rs");
}
