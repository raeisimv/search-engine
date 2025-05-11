use crate::errors::MyResult;

pub async fn write(path: &std::path::PathBuf, buf: &[u8]) -> MyResult {
    async_fs::write(path.as_path(), buf).await?;
    Ok(())
}

pub async fn read(path: &std::path::PathBuf) -> MyResult<Vec<u8>> {
    let x = async_fs::read(path.as_path()).await?;
    Ok(x)
}

pub async fn delete(path: &std::path::PathBuf) -> MyResult {
    async_fs::remove_file(path).await?;
    Ok(())
}
