use cid::Cid;

use crate::cli::config::{Config, ConfigError};
use crate::device::DeviceError;
use crate::types::DorStore;

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
        if device.remote_stat(object.cid()).await?.is_some() {
            continue;
        }
        let cid = device.push(&working_dir.join(path)).await?;
        if cid != *object.cid() {
            return Err(PushError::CidMismatch(cid, *object.cid()));
        }
    }

    let new_root_cid = device.push_dor_store(next_base).await?;

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
    #[error("no changes to push")]
    NoChanges,
    #[error("missmatched root cid: {0} != {1}")]
    MissmatchedRootCid(Cid, Cid),
    #[error("missmatched base: {0:?} != {1:?}")]
    MissmatchedBase(DorStore, DorStore),
}
