use std::path::PathBuf;

use rfd::AsyncFileDialog;

pub async fn pick_project_directory() -> Option<PathBuf> {
    let file_handle = AsyncFileDialog::new().pick_folder().await?;

    Some(file_handle.path().into())
}
