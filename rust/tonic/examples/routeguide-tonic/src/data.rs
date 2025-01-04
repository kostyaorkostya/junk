use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Point {
    latitude: i32,
    longitude: i32,
}

#[derive(Debug, Deserialize)]
struct Feature {
    name: String,
    location: Point,
}

pub fn load() -> Vec<crate::routeguide::Feature> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
}
