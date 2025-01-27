use axum::async_trait;
use axum::extract::multipart::Field;
use tokio::fs;

use mtc_model::storage_model::{StorageModel, StoragesModel};

use crate::error::Result;

crate::impl_service!(StorageService);

#[async_trait]
pub trait StorageTrait {
    fn get_dir_path(&self, dir: &str) -> String;
    fn get_file_path(&self, dir: &str, file: &str) -> String;
    fn get_private_dir_path(&self, dir: &str) -> String;
    fn get_private_file_path(&self, dir: &str, file: &str) -> String;

    async fn get_dir(&self, path: &str) -> Result<StoragesModel>;
    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool>;
    async fn is_file_exists(&self, path: &str) -> Result<bool>;
    async fn remove_dir(&self, path: &str) -> Result<bool>;
    async fn save_file(&self, path: &str, data: Field<'_>) -> Result<()>;
    async fn delete_file(&self, path: &str) -> Result<()>;
    async fn create_assets(&self, id: &str) -> Result<()>;
    async fn delete_assets(&self, id: &str) -> Result<()>;
}

#[async_trait]
impl StorageTrait for StorageService {
    fn get_dir_path(&self, dir: &str) -> String {
        [self.cfg.storage_path.as_str(), dir].join("/")
    }

    fn get_file_path(&self, dir: &str, file: &str) -> String {
        [self.cfg.storage_path.as_str(), dir, file].join("/")
    }

    fn get_private_dir_path(&self, dir: &str) -> String {
        [self.cfg.private_storage_path.as_str(), dir].join("/")
    }

    fn get_private_file_path(&self, dir: &str, file: &str) -> String {
        [self.cfg.private_storage_path.as_str(), dir, file].join("/")
    }

    async fn get_dir(&self, path: &str) -> Result<StoragesModel> {
        let mut storages = StoragesModel::default();

        if let Ok(mut folder) = fs::read_dir(path).await {
            while let Ok(Some(child)) = folder.next_entry().await {
                if let Ok(meta) = child.metadata().await {
                    if meta.is_file() {
                        storages.files.push(StorageModel {
                            name: child.file_name().into_string().unwrap_or_default(),
                            size: meta.len() as usize,
                        })
                    }
                }
            }
        }

        Ok(storages)
    }

    async fn is_dir_exists_or_create(&self, path: &str) -> Result<bool> {
        match fs::metadata(&path).await {
            Ok(value) => Ok(value.is_dir()),
            Err(_) => Ok(fs::create_dir_all(path).await.is_ok()),
        }
    }

    async fn is_file_exists(&self, path: &str) -> Result<bool> {
        match fs::metadata(path).await {
            Ok(value) => Ok(value.is_file()),
            Err(_) => Ok(false),
        }
    }

    async fn remove_dir(&self, path: &str) -> Result<bool> {
        Ok(fs::remove_dir_all(path).await.is_ok())
    }

    async fn save_file(&self, path: &str, data: Field<'_>) -> Result<()> {
        let file_path = [path, data.file_name().unwrap()].join("/");

        fs::write(file_path, data.bytes().await?).await?;
        Ok(())
    }

    async fn delete_file(&self, path: &str) -> Result<()> {
        fs::remove_file(path).await?;

        Ok(())
    }

    async fn create_assets(&self, id: &str) -> Result<()> {
        self.is_dir_exists_or_create(&self.get_dir_path(id)).await?;
        self.is_dir_exists_or_create(&self.get_private_dir_path(id))
            .await?;
        Ok(())
    }

    async fn delete_assets(&self, id: &str) -> Result<()> {
        self.remove_dir(&self.get_dir_path(id)).await?;
        self.remove_dir(&self.get_private_dir_path(id)).await?;
        Ok(())
    }
}
