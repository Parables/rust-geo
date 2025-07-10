use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::models::{AlternateNameEntry, GeoName};

pub fn parse_geonames_file(path: &Path) -> io::Result<impl Iterator<Item = io::Result<GeoName>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    Ok(std::iter::from_fn(move || {
        let line_result = lines.next()?;
        Some(line_result.and_then(|line| {
            let mut parts_iter = line.split('\t');

            let geoname_id = parts_iter
                .next()
                .unwrap_or("")
                .parse::<i64>()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid geoname_id: {}", e),
                    )
                })?;
            let name = parts_iter.next().unwrap_or("").to_string();
            parts_iter.next();
            parts_iter.next();
            parts_iter.next();
            parts_iter.next();
            let feature_class = parts_iter
                .next()
                .unwrap_or(" ")
                .chars()
                .next()
                .unwrap_or(' ');
            let feature_code = parts_iter.next().unwrap_or("").to_string();
            let country_code = parts_iter.next().unwrap_or("").to_string();
            parts_iter.next();
            let admin1_code = parts_iter.next().unwrap_or("").to_string();

            Ok(GeoName {
                geoname_id,
                name,
                feature_class,
                feature_code,
                country_code,
                admin1_code,
            })
        }))
    }))
}

pub fn parse_alternate_names_file(
    path: &Path,
) -> io::Result<impl Iterator<Item = io::Result<AlternateNameEntry>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    Ok(std::iter::from_fn(move || {
        let line_result = lines.next()?;
        Some(line_result.and_then(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Malformed line in alternateNamesV2.txt (too few fields): {}",
                        line
                    ),
                ));
            }

            let alternate_name_id = parts[0].parse::<i64>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid alternate_name_id: {}", e),
                )
            })?;
            let geoname_id = parts[1].parse::<i64>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid geoname_id in alternateNamesV2.txt: {}", e),
                )
            })?;
            let isolanguage = parts[2].to_string();
            let alternate_name = parts[3].to_string();

            let is_preferred_name = parts
                .get(4)
                .and_then(|s| s.parse::<i32>().ok())
                .map(|v| v == 1);
            let is_short_name = parts
                .get(5)
                .and_then(|s| s.parse::<i32>().ok())
                .map(|v| v == 1);
            let is_colloquial = parts
                .get(6)
                .and_then(|s| s.parse::<i32>().ok())
                .map(|v| v == 1);
            let is_historic = parts
                .get(7)
                .and_then(|s| s.parse::<i32>().ok())
                .map(|v| v == 1);
            let from = parts.get(8).map(|s| s.to_string());
            let to = parts.get(9).map(|s| s.to_string());

            Ok(AlternateNameEntry {
                alternate_name_id,
                geoname_id,
                isolanguage,
                alternate_name,
                is_preferred_name,
                is_short_name,
                is_colloquial,
                is_historic,
                from,
                to,
            })
        }))
    }))
}
