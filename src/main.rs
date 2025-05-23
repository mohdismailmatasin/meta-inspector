mod lib;

use metadata_extractor::{get_audio_metadata, get_image_metadata, get_video_metadata};

pub fn main() {
    let file_path = std::env::args().nth(1).expect("Usage: metadata_extractor <file_path>");
    let path = std::path::Path::new(&file_path);
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let metadata = if ["jpg", "jpeg", "png", "tiff", "bmp"].contains(&ext.as_str()) {
        get_image_metadata(&file_path)
    } else if ["mp3", "flac", "wav", "ogg", "m4a", "aac"].contains(&ext.as_str()) {
        get_audio_metadata(&file_path)
    } else if ["mp4", "mkv", "avi", "mov", "webm", "flv"].contains(&ext.as_str()) {
        get_video_metadata(&file_path)
    } else {
        serde_json::json!({"error": "Unsupported file type"})
    };

    // Print in a simple, readable way
    if let serde_json::Value::Object(map) = &metadata {
        for (k, v) in map {
            println!("{}: {}", k, v);
        }
    } else {
        println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
    }

    // Export to Desktop as meta_<file/source name>.txt
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let export_path = format!("/home/mohdismailmatasin/Desktop/meta_{}.txt", file_stem);
    let mut file = std::fs::File::create(&export_path).expect("Unable to create export file");
    if let serde_json::Value::Object(map) = &metadata {
        let max_key_len = map.keys().map(|k| k.len()).max().unwrap_or(0);
        for (k, v) in map {
            writeln!(file, "{:<width$} : {}", k, v, width = max_key_len).ok();
        }
    } else {
        writeln!(file, "{}", serde_json::to_string_pretty(&metadata).unwrap()).ok();
    }
    writeln!(file, "\n==============================").ok();
    println!("Exported metadata to {}", export_path);
}
