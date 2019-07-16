extern crate protoc_grpcio;

use protoc_grpcio::compile_grpc_protos;

fn main() {
    let proto_root = "helm/_proto";
    let output_dir = "src";
    // println!("cargo:rerun-if-changed={}", proto_root);
    compile_grpc_protos(
        &[
            "hapi/chart/chart.proto",
            "hapi/chart/config.proto",
            "hapi/chart/metadata.proto",
            "hapi/chart/template.proto",
            "hapi/release/hook.proto",
            "hapi/release/info.proto",
            "hapi/release/release.proto",
            "hapi/release/status.proto",
            "hapi/release/test_run.proto",
            "hapi/release/test_suite.proto",
            "hapi/rudder/rudder.proto",
            "hapi/services/tiller.proto",
            "hapi/version/version.proto",
        ],
        &[proto_root],
        &output_dir,
        None,
    )
    .expect("Failed to compile helm proto definitions");
}
