use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use cid::Cid;

use crate::cli::changes::ChangeLog;
use crate::cli::config::{Config, ConfigError};
use crate::device::{Device, DeviceError};

pub async fn file_needs_pull(
    device: &Device,
    path: &PathBuf,
    cid: &Cid,
) -> Result<bool, PullError> {
    if !path.exists() {
        return Ok(true);
    } else if path.is_dir() {
        return Err(PullError::PathIsDirectory(path.clone()));
    }

    let file = File::open(path)?;
    let hash = device.hash_ipfs_data(file, false).await?;
    if hash == *cid {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn pull_file(device: &Device, cid: &Cid, path: &PathBuf) -> Result<(), PullError> {
    // TODO: replace with gateway read
    let data = device.read_ipfs_gateway_data(cid, None).await?;
    let mut object_path = path.clone();
    object_path.pop();
    std::fs::create_dir_all(object_path)?;
    let mut file = std::fs::File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

pub async fn pull(config: &Config) -> Result<(), PullError> {
    let on_disk_device = config.on_disk_device()?;
    let alias = on_disk_device.alias();
    let base_root_cid = Config::root_cid(config)?;
    let base_dor_store = Config::base(config)?;
    let device = config.device()?;

    let root_cid = device.read_root_cid().await?;
    if root_cid == base_root_cid {
        tracing::info!("root cid is up to date");
    } else {
        config.set_root_cid(&root_cid)?;
    }

    let mut dor_store = base_dor_store.clone();
    if root_cid != Cid::default() {
        tracing::info!("root cid is not set");
        dor_store = device.read_dor_store(&root_cid, true).await?;
    }

    if dor_store == base_dor_store {
        tracing::info!("dor store is up to date");
    } else {
        config.set_base(&dor_store)?;
    }

    let objects = dor_store.objects();

    for (path, object) in objects.iter() {
        let working_path = config.working_dir().join(path);
        if !file_needs_pull(&device, &working_path, object.cid()).await? {
            continue;
        }

        // TODO: this should use the gateway
        pull_file(&device, object.cid(), &working_path).await?;
    }

    let change_log = ChangeLog::new(alias, &dor_store, &root_cid);
    config.set_change_log(change_log)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PullError {
    #[error("config error")]
    Config(#[from] ConfigError),
    #[error("cid error: {0}")]
    Cid(#[from] cid::Error),
    #[error("device error: {0}")]
    Device(#[from] DeviceError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("path is a directory")]
    PathIsDirectory(PathBuf),
}
