use std::{ffi::OsStr, path::Path};

use reqwest::{multipart::Part, Body};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn part_from_file_path<P: AsRef<Path>>(
    file_path: P,
) -> Result<Part, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let file_path = file_path.as_ref();
    let file_name = file_path
        .file_name()
        .map(|filename| filename.to_string_lossy().into_owned());
    let file_extension = file_path.extension().and_then(OsStr::to_str).unwrap_or("");
    let mime = mime_guess::from_ext(file_extension).first_or_octet_stream();

    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    let part = Part::stream_with_length(body, metadata.len()).mime_str(mime.as_ref())?;

    Ok(if let Some(file_name) = file_name {
        part.file_name(file_name)
    } else {
        part
    })
}
