use std::sync::Arc;
use std::error::Error;

use grpcio::{ChannelBuilder, EnvBuilder, CallOption};

use helm_api::status::Status_Code;
use helm_api::tiller;
use helm_api::tiller_grpc;
use futures::Stream;
use grpcio::MetadataBuilder;

const HELM_VERSION: &'static str = "2.13.1";
const TILLER_HOST: &'static str = "localhost:44134";

fn main() -> Result<(), Box<Error>> {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(TILLER_HOST);

    let client = tiller_grpc::ReleaseServiceClient::new(channel);
    let mut req = tiller::ListReleasesRequest::new();
    req.status_codes = vec!(
        Status_Code::UNKNOWN,
        Status_Code::DEPLOYED,
        Status_Code::DELETED,
        Status_Code::DELETING,
        Status_Code::FAILED,
        Status_Code::PENDING_INSTALL,
        Status_Code::PENDING_UPGRADE,
        Status_Code::PENDING_ROLLBACK,
    );
    let mut m = MetadataBuilder::with_capacity(1);
    m.add_str("x-helm-api-client", HELM_VERSION)?;
    let opts = CallOption::default().headers(m.build());
    for response in client.list_releases_opt(&req, opts)?.wait() {
        for release in  response?.releases.into_iter() {
            let info = release.info.unwrap() ;
            let status = info.status.unwrap() ;
            println!("{} | {} | {:?} | {}", release.name, release.version, status.code, release.namespace);
        }
    }
    Ok(())
}
