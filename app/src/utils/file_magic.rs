use std::path::Path;
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone)]
pub struct DetectedFileType {
    pub mime: String,
    pub extension: String,
}

#[derive(Debug, Clone, Copy)]
pub enum MimeGroup {
    Image,
    Audio,
    Video,
}

impl MimeGroup {
    fn prefix(self) -> &'static str {
        match self {
            Self::Image => "image/",
            Self::Audio => "audio/",
            Self::Video => "video/",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
        }
    }
}

fn normalize_extension(extension: &str) -> &str {
    match extension {
        "jpeg" => "jpg",
        _ => extension,
    }
}

const MAGIC_PREFIX_BYTES: usize = 8 * 1024;

async fn read_magic_prefix(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| format!("Failed to open uploaded file: {}", e))?;

    let mut prefix = vec![0_u8; MAGIC_PREFIX_BYTES];
    let bytes_read = file
        .read(&mut prefix)
        .await
        .map_err(|e| format!("Failed to read uploaded file: {}", e))?;

    if bytes_read == 0 {
        return Err("Uploaded file is empty".to_string());
    }

    prefix.truncate(bytes_read);
    Ok(prefix)
}

pub fn detect_file_type(data: &[u8]) -> Result<DetectedFileType, String> {
    let kind =
        infer::get(data).ok_or_else(|| "Unsupported or unrecognized file type".to_string())?;

    Ok(DetectedFileType {
        mime: kind.mime_type().to_string(),
        extension: normalize_extension(kind.extension()).to_string(),
    })
}

pub async fn detect_file_type_from_path(path: &Path) -> Result<DetectedFileType, String> {
    let data = read_magic_prefix(path).await?;
    detect_file_type(&data)
}

pub fn detect_file_type_for_group(
    data: &[u8],
    expected_group: MimeGroup,
) -> Result<DetectedFileType, String> {
    let detected = detect_file_type(data)?;

    if !detected.mime.starts_with(expected_group.prefix()) {
        return Err(format!(
            "Expected an {} file, but detected {}",
            expected_group.name(),
            detected.mime
        ));
    }

    Ok(detected)
}

pub async fn detect_file_type_for_group_from_path(
    path: &Path,
    expected_group: MimeGroup,
) -> Result<DetectedFileType, String> {
    let data = read_magic_prefix(path).await?;
    detect_file_type_for_group(&data, expected_group)
}
