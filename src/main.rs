use icechunk::{Repository, RepositoryConfig, repository::VersionInfo};
use zarrs_icechunk::AsyncIcechunkStore;
use zarrs::group::Group;
use std::env;
use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1)
        .ok_or("Usage: program <path>")?;
    let storage = icechunk::new_local_filesystem_storage(Path::new(path)).await?;
    let config = RepositoryConfig::default();
    let repo = Repository::open(Some(config), storage, HashMap::new()).await?;
    let version_info = VersionInfo::BranchTipRef("main".to_string());
    let session = repo.readonly_session(&version_info).await?;
    let store = Arc::new(AsyncIcechunkStore::new(session));
    let group = Group::async_open(store.clone(), "/meta").await?;
    let arrays = &group.async_child_arrays().await?;
    Ok(())
}
