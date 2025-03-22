use proc_macro::TokenStream;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_asset_map(input: TokenStream) -> TokenStream {
    let input_path = parse_macro_input!(input as LitStr).value();

    let call_site_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let assets_dir = Path::new(&call_site_dir).join(input_path);

    let mut map_builder = phf_codegen::Map::new();
    let mut found_files = false;

    if assets_dir.exists() && assets_dir.is_dir() {
        found_files |= process_directory(&assets_dir, &assets_dir, &mut map_builder);
    } else {
        panic!("Assets directory not found: {:?}", assets_dir);
    }

    if !found_files {
        panic!("No files found in assets directory: {:?}", assets_dir);
    }

    let phf_map_code = map_builder.build().to_string();
    phf_map_code.parse().unwrap()
}

fn process_directory(
    base_dir: &Path,
    dir: &Path,
    map_builder: &mut phf_codegen::Map<String>,
) -> bool {
    let mut found_files = false;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry_result in entries {
            if let Ok(entry) = entry_result {
                let path = entry.path();

                if path.is_dir() {
                    found_files |= process_directory(base_dir, &path, map_builder);
                } else {
                    let is_br = path.extension().map_or(false, |ext| ext == "br");
                    let file_path = path.to_string_lossy().to_string();

                    let rel_path = path.strip_prefix(base_dir).unwrap_or(&path);
                    let mut key = rel_path.to_string_lossy().to_string();

                    if is_br {
                        if let Some(stem) = rel_path.file_stem() {
                            key = PathBuf::from(rel_path.parent().unwrap_or(Path::new("")))
                                .join(stem)
                                .to_string_lossy()
                                .to_string();
                        }
                    }

                    key = key.replace('\\', "/");
                    if key.starts_with('/') {
                        key = key[1..].to_string();
                    }

                    let content_type = if is_br {
                        let orig_path = Path::new(&key);
                        mime_guess::from_path(orig_path)
                            .first_raw()
                            .unwrap_or("application/octet-stream")
                    } else {
                        mime_guess::from_path(&path)
                            .first_raw()
                            .unwrap_or("application/octet-stream")
                    };

                    let include_expr = format!(
                        "(include_bytes!(\"{}\"), \"{}\", {})",
                        file_path.replace('\\', "/"),
                        content_type,
                        if is_br { "true" } else { "false" }
                    );

                    map_builder.entry(key, &include_expr);
                    found_files = true;
                }
            }
        }
    }

    found_files
}
