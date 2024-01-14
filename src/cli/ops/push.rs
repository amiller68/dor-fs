use std::fs::File;
use std::path::PathBuf;

use cid::Cid;

use crate::cli::config::{Config, ConfigError};
use crate::cli::device::{Device, DeviceError};
use crate::types::{Manifest, Object};

/// Push a file to the remote ipfs node
pub async fn push_file(device: &Device, file_path: &PathBuf, object: &Object) -> Result<Cid, PushError> {
    // See if the cid already exists on the remote
    if device.stat_ipfs_data(object.cid(), true).await?.is_some() {
        println!("Skipping {} as it already exists", file_path.display());
        return Ok(*object.cid());
    };
    println!("Pushing {}", file_path.display());
    let file = File::open(file_path)?;
    let cid = device.write_ipfs_data(file, true).await?;
    Ok(cid)
}

pub async fn push(config: &Config) -> Result<(), PushError> {
    let working_dir = config.working_dir().clone();
    let device = config.device()?;
    let disk_root_cid = config.root_cid()?;

    let disk_base = config.base()?;
    let change_log = config.change_log()?;
    let (root_cid, base) = change_log.first_version().unwrap();
    let (next_root_cid, next_base) = change_log.last_version().unwrap();

    // Check our root matches our on-disk root
    if root_cid != &disk_root_cid {
        return Err(PushError::MissmatchedRootCid(*root_cid, disk_root_cid));
    }

    // Check our base matches our on-disk base
    if base != &disk_base {
        return Err(PushError::MissmatchedBase(base.clone(), disk_base));
    }

    // Check our next_root_cid matches our on-disk root
    if root_cid == next_root_cid {
        return Err(PushError::NoChanges);
    }

    // Double Check our next_base matches our on-disk base
    if base == next_base {
        return Err(PushError::NoChanges);
    }

    let objects = next_base.objects();

    // Tell the remote to pin all the objects
    for (path, object) in objects.iter() {
        let mut attempts = 0;
        let tries = 5;
        for attempt in 0..tries {
            // Note: Infura has rate limits, so we need to sleep here
            std::thread::sleep(std::time::Duration::from_secs(1 + 2 ^ attempt));
            let cid = match push_file(&device, &working_dir.join(path), object).await {
                Ok(cid) => cid,
                Err(e) => {
                    println!("Error pinning {}: {}", path.display(), e);
                    println!("Retrying...");
                    continue;
                }
            };
            if cid != *object.cid() {
                return Err(PushError::CidMismatch(cid, *object.cid()));
            }
            attempts = attempt;
            break;
        }
        if attempts == tries - 1 {
            return Err(PushError::PushFailed);
        }
    }

    // Write the dor store against the remote
    let new_root_cid = device.write_manifest(next_base, true).await?;

    println!("Updating root cid from {} to {}", root_cid, new_root_cid);
    // Push the new root cid to the eth client
    device.update_root_cid(*root_cid, new_root_cid).await?;
    let mut change_log = change_log.clone();
    change_log.wipe(next_base, &new_root_cid);
    config.set_root_cid(&new_root_cid)?;
    config.set_base(next_base)?;
    config.set_change_log(change_log)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("device error: {0}")]
    Device(#[from] DeviceError),
    #[error("cid mismatch: {0} != {1}")]
    CidMismatch(Cid, Cid),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("no changes to push")]
    NoChanges,
    #[error("missmatched root cid: {0} != {1}")]
    MissmatchedRootCid(Cid, Cid),
    #[error("missmatched base: {0:?} != {1:?}")]
    MissmatchedBase(Manifest, Manifest),
    #[error("push failed")]
    PushFailed,
}
