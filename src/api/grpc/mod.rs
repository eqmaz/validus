pub mod launch;

// This brings in the generated code from OUT_DIR
// tonic::include_proto!("myservice"); // This uses the default OUT_DIR
// pub mod myservice {
//     include!("generated/myservice.rs");
// }
#[path = "generated/myservice.rs"]
pub mod myservice;
