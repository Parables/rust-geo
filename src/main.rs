use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::OnceLock;
use zip::ZipArchive; // Use OnceLock for Rust 1.70+

mod models;
mod parsers;

const GEO_STORAGE_DIR: &str = "../output";
const BASE_DOWNLOAD_URL: &str = "http://download.geonames.org/export/dump/";
static HARDCODED_GEONAMES: OnceLock<HashMap<i64, String>> = OnceLock::new();

fn download_file(file_name: &str, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let remote_url = format!("{}{}", BASE_DOWNLOAD_URL, file_name);
    let local_path = Path::new(GEO_STORAGE_DIR).join(file_name);

    println!("[INFO] Attempting to download: {}", remote_url);

    if local_path.exists() {
        if overwrite {
            println!(
                "[INFO] Local file '{}' exists. Overwriting...",
                local_path.display()
            );

            fs::remove_file(&local_path)?;
        } else {
            println!(
                "[INFO] Local file '{}' already exists and overwrite is false. Skipping download.",
                local_path.display()
            );

            return Ok(());
        }
    }
    println!(
        "[INFO] Download started: {} to {}",
        remote_url,
        local_path.display()
    );

    let mut response = reqwest::blocking::get(&remote_url)?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download {}: HTTP status {}",
            remote_url,
            response.status()
        )
        .into());
    }

    let mut dest_file = fs::File::create(&local_path)?;
    response.copy_to(&mut dest_file)?;

    println!("[INFO] Successfully downloaded: {}", local_path.display());
    Ok(())
}

fn unzip_file(
    zip_file_name: &str,
    overwrite: bool,
) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let zip_path = Path::new(GEO_STORAGE_DIR).join(zip_file_name);
    println!("[INFO] Attempting to unzip: {}", zip_path.display());

    if !zip_path.exists() {
        return Err(format!("Zip file not found: {}", zip_path.display()).into());
    }

    let file = fs::File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut extracted_files = Vec::new();

    for i in 0..archive.len() {
        let mut file_in_zip = archive.by_index(i)?;
        let file_name = file_in_zip.name();

        // Skip directories
        if file_name.ends_with('/') {
            println!("[INFO] Skipping directory: {}", file_name);
            continue;
        }

        let extracted_path = Path::new(GEO_STORAGE_DIR).join(file_name);

        // Create parent directories if they don't exist
        if let Some(parent) = extracted_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Check if file already exists
        if extracted_path.exists() {
            if overwrite {
                println!(
                    "[INFO] Extracted file '{}' exists. Overwriting...",
                    extracted_path.display()
                );
                fs::remove_file(&extracted_path)?;
            } else {
                println!(
                    "[INFO] Extracted file '{}' already exists. Skipping extraction.",
                    extracted_path.display()
                );
                extracted_files.push(extracted_path);
                continue;
            }
        }

        // Extract the file
        let mut buffer = Vec::new();
        file_in_zip.read_to_end(&mut buffer)?;
        fs::write(&extracted_path, &buffer)?;

        println!(
            "[INFO] Successfully extracted: {}",
            extracted_path.display()
        );

        extracted_files.push(extracted_path);
    }

    println!(
        "[INFO] Successfully unzipped {} files from: {}",
        extracted_files.len(),
        zip_path.display()
    );

    Ok(extracted_files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Ensuring storage directory exists...");
    let storage_path = Path::new(GEO_STORAGE_DIR);
    if !storage_path.exists() {
        println!("[INFO] Creating storage directory: {}", GEO_STORAGE_DIR);
        match fs::create_dir_all(storage_path) {
            Ok(_) => println!("[INFO] Storage directory created."),
            Err(e) => {
                eprintln!(
                    "[ERROR] Failed to create storage directory {}: {}",
                    storage_path.display(),
                    e
                );
                return Err(e.into());
            }
        }
    } else {
        println!("[INFO] Storage directory already exists.");
    }

    let files_to_parse = vec!["allCountries.zip", "alternateNamesV2.zip"];

    for file_name in &files_to_parse {
        if let Err(e) = download_file(file_name, false) {
            eprintln!(
                "[ERROR] Fatal error during download of {}: {}",
                file_name, e
            );
            return Err(e);
        }
    }

    for file_name in &files_to_parse {
        if let Err(e) = unzip_file(file_name, false) {
            eprintln!("[ERROR] Fatal error during unzip of {}: {}", file_name, e);
            return Err(e);
        }
    }

    let mut geonames: HashMap<i64, String> = HashMap::new();
    let mut hierarchy: HashMap<i64, models::ExtendedGeoNameEntry> = HashMap::new();
    let mut unparented_cities: HashMap<i64, models::ExtendedGeoNameEntry> = HashMap::new();
    let mut geoname_entries: HashMap<i64, models::GeoName> = HashMap::new();
    let mut countries: HashMap<String, i64> = HashMap::new();
    let mut states: HashMap<String, i64> = HashMap::new();
    let mut cities: Vec<i64> = Vec::new();

    hierarchy.insert(
        6295630,
        models::ExtendedGeoNameEntry {
            name: "Earth".to_string(),
            children: Vec::new(),
        },
    );

    println!("[INFO] Processing alternateNamesV2.txt");
    let alternate_names_v2_txt_path = Path::new(GEO_STORAGE_DIR).join("alternateNamesV2.txt");
    let alternate_names_iterator =
        parsers::parse_alternate_names_file(&alternate_names_v2_txt_path)?;
    for alternate_name_result in alternate_names_iterator {
        let alt_name = alternate_name_result?;
        if alt_name.isolanguage == "en"
            && (alt_name.is_preferred_name == Some(true) || alt_name.is_short_name == Some(true))
        {
            geonames.insert(alt_name.geoname_id, alt_name.alternate_name.clone());
        }
    }

    let get_name = |key: i64, default: String| -> String {
        let hardcoded_geonames = HARDCODED_GEONAMES.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert(6252001, "United States of America".to_string());
            // TODO: Add more entries here
            m
        });

        if let Some(geoname) = hardcoded_geonames.get(&key) {
            return geoname.clone();
        }

        geonames.get(&key).cloned().unwrap_or(default)
    };

    println!("[INFO] Processing allCountries.txt");
    let all_countries_txt_path = Path::new(GEO_STORAGE_DIR).join("allCountries.txt");
    let geonames_iterator = parsers::parse_geonames_file(&all_countries_txt_path)?;
    for geoname_result in geonames_iterator {
        let g = geoname_result?;
        if g.is_continent() || g.is_country() || g.is_state_region() || g.is_city_town() {
            geoname_entries.insert(g.geoname_id, g.clone());
        }

        if g.is_country() {
            hierarchy
                .entry(6295630)
                .or_insert_with(|| models::ExtendedGeoNameEntry {
                    name: "Earth".to_string(),
                    children: Vec::new(),
                })
                .children
                .push(models::ChildEntry {
                    id: g.geoname_id,
                    name: get_name(g.geoname_id, g.name.clone()),
                });

            hierarchy.insert(
                g.geoname_id,
                models::ExtendedGeoNameEntry {
                    name: get_name(g.geoname_id, g.name.clone()),
                    children: Vec::new(),
                },
            );

            countries.insert(g.country_code.clone(), g.geoname_id);
        }

        if g.is_state_region() {
            hierarchy.insert(
                g.geoname_id,
                models::ExtendedGeoNameEntry {
                    name: get_name(g.geoname_id, g.name.clone()),
                    children: Vec::new(),
                },
            );

            states.insert(
                format!("{}.{}", g.country_code.clone(), g.admin1_code.clone()),
                g.geoname_id,
            );
        }

        if g.is_city_town() {
            cities.push(g.geoname_id.clone());
        }
    }

    for (_key, geoname_id) in states.iter() {
        let g = geoname_entries.get(&geoname_id).cloned();
        if let Some(g) = g {
            let p_id = countries.get(&g.country_code).cloned();
            if let Some(p_id) = p_id {
                let p = geoname_entries.get(&p_id).cloned();
                if let Some(p) = p {
                    hierarchy
                        .entry(p_id)
                        .or_insert_with(|| models::ExtendedGeoNameEntry {
                            name: get_name(p.geoname_id, p.name.clone()),
                            children: Vec::new(),
                        })
                        .children
                        .push(models::ChildEntry {
                            id: g.geoname_id,
                            name: get_name(g.geoname_id, g.name.clone()),
                        });
                }
            }
        }
    }

    for geoname_id in cities.iter() {
        let g = geoname_entries.get(&geoname_id).cloned();
        if let Some(g) = g {
            let p_id = states
                .get(&format!("{}.{}", &g.country_code, &g.admin1_code))
                .cloned();
            let c_id = countries.get(&g.country_code).cloned();
            if let Some(p_id) = p_id {
                let p = geoname_entries.get(&p_id).cloned();
                if let Some(p) = p {
                    hierarchy
                        .entry(p_id)
                        .or_insert_with(|| models::ExtendedGeoNameEntry {
                            name: get_name(p.geoname_id, p.name.clone()),
                            children: Vec::new(),
                        })
                        .children
                        .push(models::ChildEntry {
                            id: g.geoname_id,
                            name: get_name(g.geoname_id, g.name.clone()),
                        });
                }
            } else if let Some(c_id) = c_id {
                let c = geoname_entries.get(&c_id).cloned();
                if let Some(c) = c {
                    unparented_cities
                        .entry(c_id)
                        .or_insert_with(|| models::ExtendedGeoNameEntry {
                            name: get_name(c.geoname_id, c.name.clone()),
                            children: Vec::new(),
                        })
                        .children
                        .push(models::ChildEntry {
                            id: g.geoname_id,
                            name: get_name(g.geoname_id, g.name.clone()),
                        });
                }
            }
        }
    }

    let mut hierarchy_json = fs::File::create(Path::new(GEO_STORAGE_DIR).join("hierarchy.json"))?;
    hierarchy_json.write_all(serde_json::to_string_pretty(&hierarchy)?.as_bytes())?;
    println!("[INFO] Hierarchy written to: hierarchy.json");

    let mut unparented_cities_json =
        fs::File::create(Path::new(GEO_STORAGE_DIR).join("unparented_cities.json"))?;
    unparented_cities_json
        .write_all(serde_json::to_string_pretty(&unparented_cities)?.as_bytes())?;
    println!("[INFO] Cities with no states/regions written to: unparented_cities.json");

    println!("[INFO] All done... Enjoy");
    Ok(())
}
