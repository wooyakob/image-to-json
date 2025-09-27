use std::env;
use std::fs;
use std::path::Path;
use std::fmt::Write as _;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde_json::json;
use mime_guess;
use sha2::{Digest, Sha256};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse flags and image path:
    // img-to-json [--raw|--data-url] [--compact] [--sha256] [--out <path>.json] <image_path>
    let mut use_data_url = true; // default behavior
    let mut compact = false;
    let mut include_sha256 = false;
    let mut out_path: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();

    let mut it = env::args().skip(1).peekable();
    while let Some(arg) = it.next() {
        // Support both --out <path> and --out=<path>
        if let Some(val) = arg.strip_prefix("--out=") {
            out_path = Some(val.to_string());
            continue;
        }
        match arg.as_str() {
            "--raw" => use_data_url = false,
            "--data-url" => use_data_url = true,
            "--compact" => compact = true,
            "--sha256" => include_sha256 = true,
            "--out" => {
                let Some(p) = it.next() else {
                    eprintln!("Error: --out requires a path argument");
                    eprintln!("Usage: img-to-json [--raw|--data-url] [--compact] [--sha256] [--out <path>.json] <image_path>");
                    eprintln!("Examples:\n  img-to-json --data-url ./photo.png\n  img-to-json --raw --compact --out ./photo.json ./photo.png\n  img-to-json --sha256 ./photo.png");
                    std::process::exit(2);
                };
                out_path = Some(p.to_string());
            }
            other => positional.push(other.to_string()),
        }
    }

    if positional.len() != 1 {
        eprintln!("Usage: img-to-json [--raw|--data-url] [--compact] [--sha256] [--out <path>.json] <image_path>");
        eprintln!("Examples:\n  img-to-json --data-url ./photo.png\n  img-to-json --raw --compact --out ./photo.json ./photo.png\n  img-to-json --sha256 ./photo.png");
        std::process::exit(2);
    }
    let img_path = positional.remove(0);

    let path = Path::new(&img_path);
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()).into());
    }
    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()).into());
    }

    let data = fs::read(path)?;
    let encoded = STANDARD.encode(&data);

    let mime = mime_guess::from_path(path)
        .first_or_octet_stream()
        .essence_str()
        .to_string();

    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let size_bytes = data.len() as u64;

    let data_value = if use_data_url {
        format!("data:{};base64,{}", mime, encoded)
    } else {
        encoded
    };

    let mut obj = json!({
        "filename": file_name,
        "mime_type": mime,
        "size_bytes": size_bytes,
        "encoding": "base64",
        "data": data_value,
    });

    if include_sha256 {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let digest = hasher.finalize();
        // hex-encode lowercase
        let mut hex = String::with_capacity(digest.len() * 2);
        for b in digest {
            let _ = write!(&mut hex, "{:02x}", b);
        }
        if let Some(map) = obj.as_object_mut() {
            map.insert("sha256".to_string(), serde_json::Value::String(hex));
        }
    }

    let output = if compact {
        serde_json::to_string(&obj)?
    } else {
        serde_json::to_string_pretty(&obj)?
    };

    if let Some(path) = out_path {
        fs::write(path, output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}