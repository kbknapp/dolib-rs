pub use account::Account;
pub use action::{Action, Actions};
pub use backup::{Backup, Backups};
pub use dns::{DnsRecord, DnsRecords};
pub use domain::{Domain, Domains};
pub use droplet::{Droplet, Droplets};
pub use error::DoError;
pub use header::HeaderOnly;
pub use image::{Image, Images};
pub use kernel::{Kernel, Kernels};
pub use links::Links;
pub use meta::Meta;
pub use namedresponse::NamedResponse;
pub use neighbors::Neighbors;
pub use network::{Network, Networks};
pub use page::{NewIter, Pages, RawPagedResponse};
pub use region::{Region, Regions};
pub use size::{Size, Sizes};
pub use snapshot::{Snapshot, Snapshots};
pub use ssh_key::{SshKey, SshKeys};
pub use upgrades::{DropletUpgrade, DropletUpgrades, ResponseStringArray};

mod account;
mod action;
mod backup;
mod dns;
mod domain;
mod droplet;
mod error;
// mod features;
mod header;
mod image;
mod links;
mod kernel;
mod meta;
mod neighbors;
mod network;
mod page;
mod namedresponse;
mod region;
mod size;
mod snapshot;
mod ssh_key;
mod upgrades;

pub trait NotArray {}
