use kube::api::GroupVersionKind;
use kube::Discovery;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(context(false), display("Kubernetes error"))]
    KubeError { source: kube::Error },
}

/// This can be used to interact with Stackable components in various ways.
pub struct StackableApi {
    client: kube::Client,
}

const GROUP_FILTER: &[&str] = &[
    "airflow.stackable.tech",
    "druid.stackable.tech",
    "hbase.stackable.tech",
    "hdfs.stackable.tech",
    "hive.stackable.tech",
    "kafka.stackable.tech",
    "nifi.stackable.tech",
    "spark.stackable.tech",
    "superset.stackable.tech",
    "trino.stackable.tech",
    "zookeeper.stackable.tech",
    "authentication.stackable.tech",
    "s3.stackable.tech",
    "secrets.stackable.tech",
    "opa.stackable.tech",
    "listeners.stackable.tech",
];

impl StackableApi {
    pub fn new(client: kube::Client) -> Self {
        Self { client }
    }

    /// This returns a list of all Stackable CRDs that are installed in the current cluster/context.
    /// This does not yet mean that they are in use anywhere, i.e. that there are any operators installed for these CRDs or any products deployed.
    ///
    /// Results are not currently cached!
    pub async fn get_installed_stackable_crds(&self) -> Result<Vec<GroupVersionKind>, Error> {
        let discovery = Discovery::new(self.client.clone())
            .filter(GROUP_FILTER)
            .run()
            .await?;

        let mut gvks = Vec::new();

        for group in discovery.groups() {
            for version in group.versions() {
                for (api_resource, _) in group.versioned_resources(version) {
                    let gvk = GroupVersionKind::gvk(group.name(), version, &api_resource.kind);
                    gvks.push(gvk);
                }
            }
        }

        Ok(gvks)
    }
}
