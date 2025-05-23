use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use md5;
use chrono::{DateTime, Local};
use serde_json::Value;
use std::io::BufReader;
use lofty::file::TaggedFileExt;
use exif::Reader;

#[pyfunction]
fn extract_metadata(file_path: &str) -> PyResult<String> {
    // Call your Rust metadata extraction logic here
    let ext = Path::new(file_path).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let metadata = if ["jpg", "jpeg", "png", "tiff", "bmp"].contains(&ext.as_str()) {
        crate::get_image_metadata(file_path)
    } else if ["mp3", "flac", "wav", "ogg", "m4a", "aac"].contains(&ext.as_str()) {
        crate::get_audio_metadata(file_path)
    } else if ["mp4", "mkv", "avi", "mov", "webm", "flv"].contains(&ext.as_str()) {
        crate::get_video_metadata(file_path)
    } else {
        serde_json::json!({"error": "Unsupported file type"})
    };
    Ok(serde_json::to_string_pretty(&metadata).unwrap())
}

#[pyfunction]
fn hash_file(file_path: &str) -> PyResult<(String, String)> {
    let data = fs::read(file_path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
    let md5_hash = format!("{:x}", md5::compute(&data));
    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(&data);
    let sha256_hash = format!("{:x}", hasher_sha256.finalize());
    Ok((md5_hash, sha256_hash))
}

#[pyfunction]
fn file_timestamps(file_path: &str) -> PyResult<(String, String, String)> {
    let meta = fs::metadata(file_path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
    let created = meta.created().ok().map(|t| DateTime::<Local>::from(t).to_rfc3339()).unwrap_or("N/A".to_string());
    let modified = meta.modified().ok().map(|t| DateTime::<Local>::from(t).to_rfc3339()).unwrap_or("N/A".to_string());
    let accessed = meta.accessed().ok().map(|t| DateTime::<Local>::from(t).to_rfc3339()).unwrap_or("N/A".to_string());
    Ok((created, modified, accessed))
}

#[pyfunction]
fn batch_process(dir_path: &str) -> PyResult<Vec<(String, String)>> {
    let mut results = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_string_lossy().to_string();
            let meta = extract_metadata(&path)?;
            results.push((path, meta));
        }
    }
    Ok(results)
}

pub fn get_image_metadata(path: &str) -> serde_json::Value {
    let file = std::fs::File::open(path);
    if let Ok(f) = file {
        let mut bufreader = BufReader::new(f);
        match exif::Reader::new().read_from_container(&mut bufreader) {
            Ok(exif) => {
                let mut map = serde_json::Map::new();
                for f in exif.fields() {
                    map.insert(format!("{}", f.tag), serde_json::json!(f.display_value().with_unit(&exif).to_string()));
                }
                serde_json::Value::Object(map)
            },
            Err(e) => serde_json::json!({"error": format!("Failed to read EXIF: {}", e)})
        }
    } else {
        serde_json::json!({"error": "Failed to open file for EXIF reading"})
    }
}

pub fn get_audio_metadata(path: &str) -> serde_json::Value {
    match lofty::read_from_path(path) {
        Ok(tagged_file) => {
            let mut map = serde_json::Map::new();
            if let Some(tag) = tagged_file.primary_tag() {
                for item in tag.items() {
                    map.insert(format!("{:?}", item.key()), serde_json::json!(format!("{:?}", item.value())));
                }
            }
            serde_json::Value::Object(map)
        },
        Err(e) => serde_json::json!({"error": format!("Failed to read audio metadata: {}", e)})
    }
}

pub fn get_video_metadata(path: &str) -> serde_json::Value {
    let output = std::process::Command::new("ffprobe")
        .args(["-v", "quiet", "-print_format", "json", "-show_format", "-show_streams", path])
        .output();
    match output {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            serde_json::from_str(&s).unwrap_or_else(|_| serde_json::json!({"error": "Failed to parse ffprobe output"}))
        },
        Err(e) => serde_json::json!({"error": format!("Failed to run ffprobe: {}", e)})
    }
}

#[pymodule]
fn metadata_extractor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(hash_file, m)?)?;
    m.add_function(wrap_pyfunction!(file_timestamps, m)?)?;
    m.add_function(wrap_pyfunction!(batch_process, m)?)?;
    Ok(())
}
