#![allow(non_snake_case)]

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use zookeeper::{WatchedEvent, Watcher, ZkResult, ZooKeeper};

use super::cloud_methods::SolrCloudMethods;
use crate::config::SolrClientConfig;
use crate::{SolrCoreMethods, SolrResult};

// empty logging watcher, for the ZK connection
struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, _: WatchedEvent) {}
}

/// Access a solr node using live_node information store in a Zookeeper ensemble.
///
/// Live nodes are picked at random from the children of /<zk_chroot>/live_nodes, and rechosen on
/// each new request.
///
/// This takes zookeeper details as the arguments to new()
///
/// ```no_run
/// use stellr::prelude::*;
/// use stellr::ZkSolrClient;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let my_client = ZkSolrClient::new("localhost:9983", "/")?;
/// assert_eq!(my_client.live_node_url().unwrap(), "http://192.168.1.2:8983/solr");
/// # Ok(()) }
/// ```
///
/// Please see the SolrCoreMethods and SolrCloudMethods Traits for details on how to use these
/// clients.
///
/// Also note that the zookeeper connection is stored within an Arc, so clone()'ed instances will
/// share zookeeper connection instances, whereas new() will create new ZK clients each time.
///
// #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone)]
pub struct ZkSolrClient {
    zk_quorum: String,
    zk_chroot: String,
    zk_connection: Arc<Mutex<ZooKeeper>>,
    request_config: SolrClientConfig,
}

impl ZkSolrClient {
    /// Build a zookeeper based connection from zookeeper ensemble and chroot infomration
    pub fn new(zk_quorum: &str, zk_chroot: &str) -> SolrResult<ZkSolrClient> {
        Ok(ZkSolrClient {
            zk_quorum: String::from(zk_quorum),
            zk_chroot: String::from(zk_chroot),
            zk_connection: Arc::new(Mutex::new(zk_connection(zk_quorum, zk_chroot)?)),
            request_config: SolrClientConfig::new(),
        })
    }

    /// Extract the list of all current live nodes from zookeeper
    pub fn live_nodes(&self) -> ZkResult<Vec<String>> {
        {
            let zk = self.zk_connection.lock().unwrap();

            // error if live_nodes is not found
            zk.exists("/live_nodes", true)?;

            zk.get_children("/live_nodes", true)
        }
    }

    /// Extract the details of just one node from live_nodes
    pub fn pick_one_live_node(&self) -> SolrResult<String> {
        let children = self.live_nodes()?;

        let mut rng = thread_rng();
        Ok(children.choose(&mut rng).cloned().unwrap())
    }

    /// A basic method to reset dead zookeeper connections
    pub fn reset_zookeeper(&self) -> ZkResult<()> {
        // TODO: Automate the process of re-estabilishing a zookeeper connection
        {
            let mut zk = self.zk_connection.lock().unwrap();

            *zk = zk_connection(&self.zk_quorum, &self.zk_chroot)?;
        }
        Ok(())
    }
}

/// Create a zookeeper client
fn zk_connection(zk_quorum: &str, zk_chroot: &str) -> ZkResult<ZooKeeper> {
    ZooKeeper::connect(
        &full_zk_path(zk_quorum, zk_chroot),
        Duration::from_secs(20),
        LoggingWatcher,
    )
}

/// Build out the zookeeper information as a single string
fn full_zk_path(zk_quorum: &str, zk_chroot: &str) -> String {
    if zk_chroot != "/" {
        format!("{}/{}", zk_quorum, zk_chroot)
    } else {
        format!("{}/", zk_quorum)
    }
}

impl SolrCoreMethods for ZkSolrClient {
    fn live_node_url(&self) -> SolrResult<String> {
        Ok(format!(
            "http://{}",
            self.pick_one_live_node()?.replace("_", "/")
        ))
    }

    fn request_config(&self) -> &SolrClientConfig {
        &self.request_config
    }

    fn set_request_config(&mut self, request_config: SolrClientConfig) {
        self.request_config = request_config;
    }
}

// bring in all the methods from SolrCloudMethods
impl SolrCloudMethods for ZkSolrClient {}

impl Default for ZkSolrClient {
    fn default() -> Self {
        let zk_quorum_str = "localhost:9983";
        let zk_chroot_str = "/";
        ZkSolrClient {
            zk_quorum: String::from(zk_quorum_str),
            zk_chroot: String::from(zk_chroot_str),
            zk_connection: Arc::new(Mutex::new(zk_connection(zk_quorum_str, zk_chroot_str).unwrap())),
            request_config: SolrClientConfig::new(),
        }
    }
}

impl fmt::Debug for ZkSolrClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZkSolrClient")
            .field("zk_quorum", &self.zk_quorum)
            .field("zk_chroot", &self.zk_chroot)
            .field("request_config", &self.request_config)
            .finish()
    }
}

impl Eq for ZkSolrClient {
}

impl PartialEq for ZkSolrClient {
    fn eq(&self, other: &Self) -> bool {
        self.zk_quorum == other.zk_quorum &&
            self.zk_chroot == other.zk_chroot &&
            self.request_config == other.request_config
    }
}

impl Hash for ZkSolrClient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.zk_quorum.hash(state);
        self.zk_chroot.hash(state);
        self.request_config.hash(state);
    }
}

impl fmt::Display for ZkSolrClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZkSolrClient({})", full_zk_path(&self.zk_quorum, &self.zk_chroot))
    }
}
