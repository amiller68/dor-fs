use cid::Cid;

use crate::device::DeviceError;
use crate::cli::config::{Config, ConfigError};
use crate::cli::changes::ChangeLog;

pub async fn pull(config: &Config) -> Result<(), PullError> {
    let on_disk_device = config.on_disk_device()?;
    let alias = on_disk_device.alias();
    let base_root_cid = Config::root_cid(config)?;
    let base_dor_store = Config::base(config)?;
    let device = config.device()?;

    let root_cid = device.get_root_cid().await?;
    if root_cid == base_root_cid {
        tracing::info!("root cid is up to date");
    } else {
        config.set_root_cid(&root_cid)?;
    }
    
    let mut dor_store = base_dor_store.clone(); 
    if root_cid != Cid::default() {
        tracing::info!("root cid is not set");
        dor_store = device.pull_dor_store(&root_cid).await?;
    }

    if dor_store == base_dor_store {
        tracing::info!("dor store is up to date");
    } else {
        config.set_base(&dor_store)?;
    }

    let objects = dor_store.objects();

    for (path, object) in objects.iter() {
        if !device.file_needs_pull(path, object.cid()).await? {
            continue;
        }

        device.download_cid(object.cid(), &config.working_dir().join(path)).await?;
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
}
