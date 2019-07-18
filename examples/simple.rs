#[macro_use] extern crate prettytable;
extern crate chrono;

use std::sync::Arc;
use std::error::Error;

use chrono::{Utc, TimeZone};
use futures::Stream;
use grpcio::{ChannelBuilder, EnvBuilder, CallOption};
use grpcio::MetadataBuilder;
use prettytable::{Table, format};

use helm_api::status::Status_Code;
use helm_api::tiller;
use helm_api::tiller_grpc;

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

    let mut table = Table::new();
    table.add_row(row!["NAME", "REVISION", "UPDATED", "STATUS", "CHART", "APP VERSION", "NAMESPACE"]);
    table.set_format(*format::consts::FORMAT_CLEAN);

    for response in client.list_releases_opt(&req, opts)?.wait() {
        for release in  response?.releases.into_iter() {
            let info = release.get_info();
            let chart = release.get_chart();
            let metadata = chart.get_metadata();
            let last_deployed = info.get_last_deployed();
            table.add_row(row![
                release.name,
                release.version,
                Utc.timestamp(last_deployed.seconds, 0),
                format!("{:?}", info.get_status().code),
                metadata.name,
                metadata.appVersion,
                release.namespace,
            ]);
        }
    }
    table.printstd();
    Ok(())
}
