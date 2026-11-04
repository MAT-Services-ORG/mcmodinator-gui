// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use std::path::PathBuf;

#[path = "parsers/legacy.rs"] mod plegacy;
#[path = "parsers/latest.rs"] mod platest;
#[path = "builders/mod.rs"] mod bmod;
#[path = "builders/cpack.rs"] mod bcpack;

pub struct ModInfo {
	id: String,
	version: String,
	display_name: String,
	description: String,
	authors: Vec<String>,
	license: String,
	forge_version: String,
	neoforge_version: String,
	icon: PathBuf,
	paths: serde_json::Value,
	dependency: serde_json::Value
}

fn add_folder_to_zip(
	zip: &mut ZipWriter<File>,
	folder_path: &Path,
	arcname: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
	let mut count = 0;

	for entry in fs::read_dir(folder_path).map_err(|error| {
		println!("{}", format!("[ERROR] {}", error)); // ERROR HANDELING
		io::Error::new(error.kind(), error.to_string())
	})? {
		let path = entry?.path();

		if path.is_dir() {
			let relative = path.strip_prefix(folder_path)?;
			let sub_arcname = Path::new(arcname).join(relative);

			count += add_folder_to_zip(
				zip,
				&path,
				sub_arcname.to_str().unwrap(),
			)?;

		} else if path.is_file() {
			zip.start_file_from_path(Path::new(arcname).join(path.strip_prefix(folder_path)?), SimpleFileOptions::default())?;
			io::copy(&mut File::open(&path)?, zip)?;

			count += 1;
		}
	}

	Ok(count)
}

pub fn escape_toml_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{:04X}", ch as u32).unwrap();
            }
            ch => escaped.push(ch),
        }
    }

    escaped
}

pub fn json_to_toml(value: serde_json::Value) -> Result<String, Box<std::io::Error>> {
	let mut ctoml = String::new();
	if value.is_object() {
		for key in value.as_object().unwrap().keys() {
			ctoml.push_str(format!("{key}={}", value[key]).as_str());
			ctoml.push('\n');
		};
	} else {
		return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Value is not JSON.")))
	}
	Ok(ctoml)
}

pub fn import_folder(
	given_path: Option<&serde_json::value::Value>,
	zip: &mut ZipWriter<File>,
	output_name: &str,
	output_type: &str
) -> Result<(), Box<dyn std::error::Error>> {
	if let Some(path) = given_path.and_then(|x| x.as_str()) && !path.is_empty() {
		match add_folder_to_zip(
			zip,
			Path::new(path),
			output_type,
		) {
			Ok(count) => {
				println!("{}", format!("[INFO] Added {count} {output_name}")); // ERROR HANDELING
			}
			Err(error) => {
				if error
					.downcast_ref::<io::Error>()
					.is_some_and(|e| e.kind() == io::ErrorKind::NotFound)
				{
					println!("{}", format!("[WARN] {output_name} not found at provided path.")); // ERROR HANDELING
					println!("{}\n{}{}", 
						"[HELP] For more informations about CLI usage, type:",
						"       ▐",
						"modinator --help"
					);
					return Ok(());
				} else {
					println!("{}", format!("[ERROR] {error}")); // ERROR HANDELING
					return Err(error);
				}
			}
		}
	}
	Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
